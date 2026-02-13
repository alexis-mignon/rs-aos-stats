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

    def test_from_invalid_str(self):
        """Characteristic("invalid") should raise."""
        with pytest.raises(BaseException):
            Characteristic("invalid")


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
