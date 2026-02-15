from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from typing import Optional, Literal
import rs_aos_stats as aos

app = FastAPI(
    title="Age of Sigmar Damage Calculator API",
    description="Calculate combat damage probabilities for Age of Sigmar",
    version="0.1.0"
)

# Enable CORS for web app
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class CombatParams(BaseModel):
    """
    Combat parameters for damage calculation.

    Characteristic Format Specification:
    - Fixed value: String containing integer (e.g., "1", "10", "40")
    - Simple dice: "D3" or "D6" (exactly as shown, case-sensitive)
    - Multiple dice: "<N>D3" or "<N>D6" where N is 1-40 (e.g., "2D6", "3D3")
    - Dice with modifier: "<N>D<faces>+<M>" where N is 1-40, faces is 3 or 6, M is 0-20
      (e.g., "2D6+3", "D3+1")

    All string formats are case-sensitive. No spaces allowed in characteristic strings.
    """
    attacks_characteristic: str = Field(
        ...,
        description="Number of attacks. Format: Fixed value '1'-'40', 'D3', 'D6', 'ND3', 'ND6', 'ND3+M', or 'ND6+M'",
        examples=["10", "D6", "2D6", "2D6+3"]
    )
    to_hit: int = Field(
        ...,
        ge=2,
        le=6,
        description="To Hit roll target. Integer between 2 and 6 (inclusive). Lower is better."
    )
    to_wound: int = Field(
        ...,
        ge=2,
        le=6,
        description="To Wound roll target. Integer between 2 and 6 (inclusive). Lower is better."
    )
    rend: int = Field(
        ...,
        ge=0,
        description="Rend/AP value. Non-negative integer (0 means no rend)."
    )
    damage_characteristic: str = Field(
        ...,
        description="Damage per hit. Format: Fixed value '1'-'10', 'D3', 'D6', 'ND3', 'ND6', 'ND3+M', or 'ND6+M'",
        examples=["1", "D3", "D6", "2D6"]
    )
    save: int = Field(
        ...,
        ge=2,
        le=7,
        description="Save characteristic. Integer 2-7 (inclusive). 7 means no save. Lower is better."
    )
    ward: Optional[int] = Field(
        None,
        ge=2,
        le=7,
        description="Ward save. Integer 2-7, or null for no ward. 7 means no ward effect. Lower is better."
    )
    hit_rule_type: Literal["normal", "crit_auto_wound", "crit_mortal_wound", "crit_double_hit"] = Field(
        default="normal",
        description="Hit rule type. Must be exactly one of: 'normal', 'crit_auto_wound', 'crit_mortal_wound', 'crit_double_hit'"
    )


class ProbabilityPoint(BaseModel):
    """Single probability point in the distribution"""
    damage: int
    probability: float


class CombatResult(BaseModel):
    """Combat calculation result"""
    probabilities: list[ProbabilityPoint]
    mean_damage: float
    max_damage: int


@app.post("/api/calculate", response_model=CombatResult)
async def calculate_damage(params: CombatParams):
    """
    Calculate combat damage probabilities for Age of Sigmar.

    ## Request Format

    All parameters are required except `ward` which may be null.

    ### Characteristic String Format

    Both `attacks_characteristic` and `damage_characteristic` use the same format:

    **Pattern:** `^(\d+|D[36]|\d+D[36]|\d*D[36]\+\d+)$`

    Valid formats (case-sensitive, no spaces):
    - Fixed integer: "1", "2", "10", "40"
    - Simple die: "D3", "D6"
    - Multiple dice: "2D3", "2D6", "40D6"
    - Dice with modifier: "D3+1", "2D6+3", "3D3+10"

    **Important:**
    - 'D' must be uppercase
    - No spaces in the string
    - N (multiplier): 1-40 for attacks, 1-10 for damage
    - M (modifier): 0-20
    - Faces: Only 3 or 6 are valid

    ### Integer Parameters

    - `to_hit`: 2-6 (2+ means 2 or higher on d6, so easier)
    - `to_wound`: 2-6 (2+ means 2 or higher on d6, so easier)
    - `rend`: 0 or positive (0 = no rend)
    - `save`: 2-7 (7 = no save, 2+ = best save)
    - `ward`: 2-7 or null (7 = no ward effect, 2+ = best ward)

    ### Hit Rule Type

    Must be exactly one of these strings:
    - `"normal"` - Standard hit/wound sequence
    - `"crit_auto_wound"` - Critical hits (unmodified 6 to hit) auto-wound
    - `"crit_mortal_wound"` - Critical hits deal mortal wounds
    - `"crit_double_hit"` - Critical hits count as 2 hits

    ## Response Format

    Returns a CombatResult with:
    - `probabilities`: Array of {damage: int, probability: float} objects
    - `mean_damage`: Expected damage value (float)
    - `max_damage`: Maximum possible damage (int)

    Probabilities sum to 1.0 (within floating point precision).

    ## Error Responses

    - 400: Invalid input format or calculation error
    - 422: Request validation error (Pydantic)
    """
    try:
        # Convert characteristics to appropriate types
        # If it's a plain number, convert to int, otherwise keep as string for dice notation
        def parse_characteristic(value: str):
            try:
                return int(value)
            except ValueError:
                return value

        attacks = parse_characteristic(params.attacks_characteristic)
        damage = parse_characteristic(params.damage_characteristic)

        # Create attack stats (positional arguments: attacks, to_hit, to_wound, rend, damages)
        attack_stats = aos.AttackStats(
            attacks,
            params.to_hit,
            params.to_wound,
            params.rend,
            damage
        )

        # Normalize ward: treat 7 as no ward (consistent with web app)
        effective_ward = params.ward if params.ward is not None and params.ward < 7 else None

        # Create defense stats (positional: save, ward)
        defense_stats = aos.DefenseStats(
            params.save,
            effective_ward
        )

        # Create combat config (attack_stats, defense_stats, roll_modifier)
        config = aos.CombatConfig(attack_stats, defense_stats, None)

        # Get rule sequence using the binding's built-in helper
        rules = aos.build_standard_sequence(params.hit_rule_type, effective_ward is not None)

        # Compute damages
        result = aos.compute_damages(config, rules)

        # Convert to response format
        probabilities = [
            ProbabilityPoint(damage=damage, probability=prob)
            for damage, prob in result
        ]

        # Calculate mean and max
        mean_damage = sum(damage * prob for damage, prob in result)
        max_damage = max(damage for damage, _ in result) if result else 0

        return CombatResult(
            probabilities=probabilities,
            mean_damage=mean_damage,
            max_damage=max_damage
        )

    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Calculation error: {str(e)}")


@app.get("/")
async def root():
    """API root endpoint"""
    return {
        "name": "Age of Sigmar Damage Calculator API",
        "version": "0.1.0",
        "endpoints": {
            "POST /api/calculate": "Calculate combat damage probabilities",
            "GET /docs": "Interactive API documentation",
        }
    }


@app.get("/health")
async def health():
    """Health check endpoint"""
    return {"status": "ok"}
