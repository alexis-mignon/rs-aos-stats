import pytest
from rs_aos_stats import Characteristic, AttackStats, DefenseStats, D6


class TestCharacteristic:
    """Tests for the Characteristic class."""

    def test_from_int(self):
        """Characteristic(3) should create successfully."""
        c = Characteristic(3)
        assert c is not None

    def test_from_str_d6(self):
        """Characteristic("D6") should create successfully."""
        c = Characteristic("D6")
        assert c is not None

    def test_from_dice_roll(self):
        """Characteristic(D6()) should create successfully."""
        c = Characteristic(D6())
        assert c is not None

    def test_from_invalid_type(self):
        """Characteristic with a non-convertible type should raise."""
        with pytest.raises(BaseException):
            Characteristic([1, 2, 3])

    def test_from_negative_int(self):
        """Characteristic should reject negative integer values."""
        with pytest.raises(ValueError):
            Characteristic(-1)

    @pytest.mark.parametrize("value", ["invalid", "xD6", "D6junk", "D3+2x", "51D6"])
    def test_from_invalid_str(self, value):
        """Characteristic should reject malformed dice strings."""
        with pytest.raises(BaseException):
            Characteristic(value)


class TestAttackStats:
    """Tests for the AttackStats class."""

    def test_new_with_ints(self):
        """AttackStats with all int values should succeed."""
        stats = AttackStats(3, 3, 3, 1, 1)
        assert stats is not None

    def test_new_with_dice_str(self):
        """AttackStats with a dice string for attacks should succeed."""
        stats = AttackStats("D6", 3, 3, 1, 1)
        assert stats is not None

    def test_new_with_dice_roll_damages(self):
        """AttackStats with a D6 object for damages should succeed."""
        stats = AttackStats(3, 3, 3, 1, D6())
        assert stats is not None

    def test_new_with_invalid_attacks(self):
        """AttackStats with invalid attacks value should raise."""
        with pytest.raises(BaseException):
            AttackStats([1, 2], 3, 3, 1, 1)

    @pytest.mark.parametrize(
        "attacks,to_hit,to_wound,rend,damages",
        [
            (-1, 3, 3, 1, 1),
            (1, -1, 3, 1, 1),
            (1, 3, -1, 1, 1),
            (1, 3, 3, -1, 1),
            (1, 3, 3, 1, -1),
        ],
    )
    def test_new_with_negative_values(self, attacks, to_hit, to_wound, rend, damages):
        """AttackStats should reject negative numeric inputs."""
        with pytest.raises(ValueError):
            AttackStats(attacks, to_hit, to_wound, rend, damages)


class TestDefenseStats:
    """Tests for the DefenseStats class."""

    def test_new_without_ward(self):
        """DefenseStats without ward should succeed."""
        stats = DefenseStats(4, None)
        assert stats is not None

    def test_new_with_ward(self):
        """DefenseStats with a ward save should succeed."""
        stats = DefenseStats(4, 6)
        assert stats is not None
