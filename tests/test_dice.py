import pytest
from rs_aos_stats import DiceRoll, D6, D3, ND6, ND3, ND6Plus, ND3Plus


class TestDiceRollFromStr:
    """Tests for DiceRoll.from_str() factory method."""

    def test_from_str_d6(self):
        """DiceRoll.from_str("D6") should return a D6 instance."""
        result = DiceRoll.from_str("D6")
        assert isinstance(result, D6)

    def test_from_str_d3(self):
        """DiceRoll.from_str("D3") should return a D3 instance."""
        result = DiceRoll.from_str("D3")
        assert isinstance(result, D3)

    def test_from_str_2d6(self):
        """DiceRoll.from_str("2D6") should return an ND6 instance."""
        result = DiceRoll.from_str("2D6")
        assert isinstance(result, ND6)

    @pytest.mark.parametrize("value", ["invalid", "xD6", "D6junk", "2D6foo", "D3+2x", "0D6", "51D6", "51D3+1"])
    def test_from_str_invalid(self, value):
        """DiceRoll.from_str should reject malformed strings."""
        with pytest.raises(ValueError):
            DiceRoll.from_str(value)

    def test_from_str_accepts_fifty_dice(self):
        """The parser should accept exactly 50 dice."""
        result = DiceRoll.from_str("50D6")
        assert isinstance(result, ND6)


class TestMultiDiceBounds:
    """Tests for the 50-dice upper bound on explicit dice classes."""

    @pytest.mark.parametrize("factory,args", [
        (ND6, (51,)),
        (ND3, (51,)),
        (ND6Plus, (51, 1)),
        (ND3Plus, (51, 1)),
    ])
    def test_constructors_reject_more_than_fifty_dice(self, factory, args):
        with pytest.raises(ValueError):
            factory(*args)

    @pytest.mark.parametrize("factory,args", [
        (ND6, (50,)),
        (ND3, (50,)),
        (ND6Plus, (50, 1)),
        (ND3Plus, (50, 1)),
    ])
    def test_constructors_accept_fifty_dice(self, factory, args):
        factory(*args)


class TestD6:
    """Tests for D6 dice values and probabilities."""

    def test_d6_values_and_probas_length(self):
        """D6 should return exactly 6 outcomes."""
        d6 = D6()
        vp = d6.values_and_probas()
        assert len(vp) == 6

    def test_d6_probabilities_sum_to_one(self):
        """D6 probabilities should sum to 1.0."""
        d6 = D6()
        vp = d6.values_and_probas()
        total = sum(p for _, p in vp)
        assert abs(total - 1.0) < 1e-10

    def test_d6_uniform_probabilities(self):
        """Each D6 outcome should have probability 1/6."""
        d6 = D6()
        vp = d6.values_and_probas()
        for _, p in vp:
            assert abs(p - 1.0 / 6.0) < 1e-10

    def test_d6_values_range(self):
        """D6 values should be 1 through 6."""
        d6 = D6()
        vp = d6.values_and_probas()
        values = sorted(v for v, _ in vp)
        assert values == [1, 2, 3, 4, 5, 6]


class TestD3:
    """Tests for D3 dice values and probabilities."""

    def test_d3_values_and_probas_length(self):
        """D3 should return exactly 3 outcomes."""
        d3 = D3()
        vp = d3.values_and_probas()
        assert len(vp) == 3

    def test_d3_probabilities_sum_to_one(self):
        """D3 probabilities should sum to 1.0."""
        d3 = D3()
        vp = d3.values_and_probas()
        total = sum(p for _, p in vp)
        assert abs(total - 1.0) < 1e-10


class TestDiceRollBaseClass:
    """Tests for the DiceRoll base class."""

    def test_base_values_and_probas_raises(self):
        """Base DiceRoll.values_and_probas() should raise NotImplementedError."""
        base = DiceRoll()
        with pytest.raises(NotImplementedError):
            base.values_and_probas()

    def test_d6_is_dice_roll(self):
        """D6 should be a subclass of DiceRoll."""
        d6 = D6()
        assert isinstance(d6, DiceRoll)
