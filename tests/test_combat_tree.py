import pytest
from rs_aos_stats import (
    AttackStats, DefenseStats, CombatConfig, RollModifier, compute_damages,
    HitRule, WoundRule, SaveRule, DamagesRule, AttackCharacteristicRule,
    CritAutoWoundRule, CritMortalWoundRule, CritDoubleHitRule, WardRule,
)

STANDARD_SEQUENCE = [
    AttackCharacteristicRule(), HitRule(), WoundRule(),
    SaveRule(), DamagesRule(),
]


class TestComputeDamages:
    """Tests for the compute_damages end-to-end function."""

    def test_basic_sequence(self):
        """Standard sequence should return probabilities summing to 1."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, STANDARD_SEQUENCE)

        assert len(result) > 0
        total = sum(p for _, p in result)
        assert abs(total - 1.0) < 1e-10

    def test_result_contains_zero_damage(self):
        """Results should include a zero-damage entry (possibility of all misses)."""
        attack_stats = AttackStats(1, 4, 4, 0, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, STANDARD_SEQUENCE)

        damages = [d for d, _ in result]
        assert 0 in damages

    def test_nonzero_damage_probability(self):
        """With reasonable stats, some nonzero damage should be possible."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, STANDARD_SEQUENCE)

        nonzero = [(d, p) for d, p in result if d > 0]
        assert len(nonzero) > 0

    def test_max_damage_bounded(self):
        """Max damage should not exceed attacks * damage_per_hit."""
        attacks = 3
        damage_per_hit = 2
        attack_stats = AttackStats(attacks, 3, 3, 1, damage_per_hit)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, STANDARD_SEQUENCE)

        max_d = max(d for d, _ in result)
        assert max_d <= attacks * damage_per_hit

    def test_with_ward_save(self):
        """Ward save should reduce expected damage compared to no ward."""
        config_no_ward = CombatConfig(
            AttackStats(3, 3, 3, 1, 1), DefenseStats(4, None), None,
        )
        config_with_ward = CombatConfig(
            AttackStats(3, 3, 3, 1, 1), DefenseStats(4, 5), None,
        )

        sequence = STANDARD_SEQUENCE + [WardRule()]

        result_no_ward = compute_damages(config_no_ward, sequence)
        result_with_ward = compute_damages(config_with_ward, sequence)

        avg_no_ward = sum(d * p for d, p in result_no_ward)
        avg_with_ward = sum(d * p for d, p in result_with_ward)
        assert avg_with_ward <= avg_no_ward

    def test_with_roll_modifier(self):
        """A +1 to hit modifier should increase expected damage."""
        config_no_mod = CombatConfig(
            AttackStats(3, 4, 4, 0, 1), DefenseStats(5, None), None,
        )
        config_with_mod = CombatConfig(
            AttackStats(3, 4, 4, 0, 1), DefenseStats(5, None),
            RollModifier(1, 0, 0),
        )

        result_no_mod = compute_damages(config_no_mod, STANDARD_SEQUENCE)
        result_with_mod = compute_damages(config_with_mod, STANDARD_SEQUENCE)

        avg_no_mod = sum(d * p for d, p in result_no_mod)
        avg_with_mod = sum(d * p for d, p in result_with_mod)
        assert avg_with_mod > avg_no_mod

    def test_crit_double_hit_increases_max_damage(self):
        """CritDoubleHitRule should allow more max damage than attacks * damage."""
        attack_stats = AttackStats(2, 6, 2, 0, 1)  # 6+ to hit = crits only
        defense_stats = DefenseStats(7, None)  # No save
        config = CombatConfig(attack_stats, defense_stats, None)

        sequence = [
            AttackCharacteristicRule(), CritDoubleHitRule(),
            WoundRule(), SaveRule(), DamagesRule(),
        ]
        result = compute_damages(config, sequence)

        max_d = max(d for d, _ in result)
        # With 2 attacks and CritDoubleHitRule, max hits = 4 (each crit gives 2)
        assert max_d > 2

    def test_dice_attacks(self):
        """Using a dice string for attacks should produce valid results."""
        attack_stats = AttackStats("D6", 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, STANDARD_SEQUENCE)

        assert len(result) > 0
        total = sum(p for _, p in result)
        assert abs(total - 1.0) < 1e-10

    def test_invalid_rule_in_sequence(self):
        """Passing a non-rule object in the sequence should raise."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        with pytest.raises(BaseException):
            compute_damages(config, [42])
