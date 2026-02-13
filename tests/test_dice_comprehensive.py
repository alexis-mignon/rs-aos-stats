"""
Comprehensive tests for dice bindings.

Strategy:
1. Test ALL dice types systematically
2. Verify ranges and probability distributions
3. Cross-check with known mathematical properties
4. Test edge cases
"""

import pytest
import rs_aos_stats as rs


class TestAllDiceTypes:
    """Systematically test every exposed dice type to catch binding errors."""

    def test_d6_basic_properties(self):
        """D6 should produce values 1-6 with uniform probability."""
        dice = rs.D6()
        results = dice.values_and_probas()

        values = [v for v, p in results]
        probas = [p for v, p in results]

        assert len(values) == 6, "D6 should have 6 outcomes"
        assert min(values) == 1, "D6 minimum should be 1"
        assert max(values) == 6, "D6 maximum should be 6"
        assert abs(sum(probas) - 1.0) < 1e-9, "Probabilities should sum to 1"

    def test_d3_basic_properties(self):
        """D3 should produce values 1-3 with uniform probability."""
        dice = rs.D3()
        results = dice.values_and_probas()

        values = [v for v, p in results]
        probas = [p for v, p in results]

        assert len(values) == 3, "D3 should have 3 outcomes"
        assert min(values) == 1, "D3 minimum should be 1"
        assert max(values) == 3, "D3 maximum should be 3"
        assert abs(sum(probas) - 1.0) < 1e-9, "Probabilities should sum to 1"

    def test_nd6_ranges(self):
        """Test ND6 produces correct ranges for various N values."""
        test_cases = [
            (1, 1, 6),    # 1D6: range 1-6
            (2, 2, 12),   # 2D6: range 2-12  (ND6 bug would give max=6)
            (3, 3, 18),   # 3D6: range 3-18
            (4, 4, 24),   # 4D6: range 4-24
        ]

        for n, expected_min, expected_max in test_cases:
            dice = rs.ND6(n)
            results = dice.values_and_probas()

            values = [v for v, p in results]
            probas = [p for v, p in results]

            actual_min = min(values)
            actual_max = max(values)

            assert actual_min == expected_min, \
                f"ND6({n}) should have min {expected_min}, got {actual_min}"
            assert actual_max == expected_max, \
                f"ND6({n}) should have max {expected_max}, got {actual_max}"
            assert abs(sum(probas) - 1.0) < 1e-9, \
                f"ND6({n}) probabilities should sum to 1"

    def test_nd3_ranges(self):
        """Test ND3 produces correct ranges for various N values."""
        test_cases = [
            (1, 1, 3),    # 1D3: range 1-3
            (2, 2, 6),    # 2D3: range 2-6
            (3, 3, 9),    # 3D3: range 3-9
            (4, 4, 12),   # 4D3: range 4-12
        ]

        for n, expected_min, expected_max in test_cases:
            dice = rs.ND3(n)
            results = dice.values_and_probas()

            values = [v for v, p in results]
            probas = [p for v, p in results]

            actual_min = min(values)
            actual_max = max(values)

            assert actual_min == expected_min, \
                f"ND3({n}) should have min {expected_min}, got {actual_min}"
            assert actual_max == expected_max, \
                f"ND3({n}) should have max {expected_max}, got {actual_max}"
            assert abs(sum(probas) - 1.0) < 1e-9, \
                f"ND3({n}) probabilities should sum to 1"

    def test_nd6_plus_ranges(self):
        """Test ND6Plus combines N dice with bonus correctly."""
        test_cases = [
            (2, 0, 2, 12),    # 2D6+0: range 2-12
            (2, 1, 3, 13),    # 2D6+1: range 3-13
            (3, 2, 5, 20),    # 3D6+2: range 5-20
        ]

        for n, bonus, expected_min, expected_max in test_cases:
            dice = rs.ND6Plus(n, bonus)
            results = dice.values_and_probas()

            values = [v for v, p in results]

            assert min(values) == expected_min, \
                f"ND6Plus({n}, {bonus}) should have min {expected_min}, got {min(values)}"
            assert max(values) == expected_max, \
                f"ND6Plus({n}, {bonus}) should have max {expected_max}, got {max(values)}"

    def test_nd3_plus_ranges(self):
        """Test ND3Plus combines N dice with bonus correctly."""
        test_cases = [
            (2, 0, 2, 6),     # 2D3+0: range 2-6
            (2, 1, 3, 7),     # 2D3+1: range 3-7
            (3, 2, 5, 11),    # 3D3+2: range 5-11
        ]

        for n, bonus, expected_min, expected_max in test_cases:
            dice = rs.ND3Plus(n, bonus)
            results = dice.values_and_probas()

            values = [v for v, p in results]

            assert min(values) == expected_min, \
                f"ND3Plus({n}, {bonus}) should have min {expected_min}, got {min(values)}"
            assert max(values) == expected_max, \
                f"ND3Plus({n}, {bonus}) should have max {expected_max}, got {max(values)}"


class TestDiceFromString:
    """Test string parsing for all dice types."""

    @pytest.mark.parametrize("dice_str,expected_min,expected_max", [
        ("D6", 1, 6),
        ("D3", 1, 3),
        ("2D6", 2, 12),
        ("3D3", 3, 9),
        ("D6+1", 2, 7),
        ("D3+2", 3, 5),
        ("2D6+3", 5, 15),
        ("2D3+1", 3, 7),
    ])
    def test_parse_valid_strings(self, dice_str, expected_min, expected_max):
        """Test parsing various valid dice strings."""
        dice = rs.DiceRoll.from_str(dice_str)

        # Verify range is correct
        results = dice.values_and_probas()
        values = [v for v, p in results]

        assert min(values) == expected_min, \
            f"{dice_str} should have min {expected_min}, got {min(values)}"
        assert max(values) == expected_max, \
            f"{dice_str} should have max {expected_max}, got {max(values)}"

    def test_parse_invalid_strings(self):
        """Test that invalid strings raise errors."""
        invalid_strings = [
            "invalid",
            "D7",      # Only D3 and D6 are valid
            "2D5",     # Only D3 and D6 are valid
            "",
        ]

        for invalid in invalid_strings:
            with pytest.raises(ValueError):
                rs.DiceRoll.from_str(invalid)


class TestMathematicalProperties:
    """Test that dice follow expected mathematical properties."""

    def test_expected_value_d6(self):
        """D6 should have expected value of 3.5."""
        dice = rs.D6()
        results = dice.values_and_probas()

        expected_value = sum(v * p for v, p in results)

        assert abs(expected_value - 3.5) < 1e-9, \
            f"D6 expected value should be 3.5, got {expected_value}"

    def test_expected_value_2d6(self):
        """2D6 should have expected value of 7."""
        dice = rs.ND6(2)
        results = dice.values_and_probas()

        expected_value = sum(v * p for v, p in results)

        assert abs(expected_value - 7.0) < 1e-9, \
            f"2D6 expected value should be 7.0, got {expected_value}"

    def test_nd6_not_equal_to_nd3(self):
        """Regression test: ND6 should NOT produce ND3 results."""
        nd6 = rs.ND6(2)
        nd3 = rs.ND3(2)

        nd6_results = nd6.values_and_probas()
        nd3_results = nd3.values_and_probas()

        nd6_values = set(v for v, p in nd6_results)
        nd3_values = set(v for v, p in nd3_results)

        # ND6(2) should have values up to 12, ND3(2) only up to 6
        assert max(nd6_values) == 12, "ND6(2) max should be 12"
        assert max(nd3_values) == 6, "ND3(2) max should be 6"

        # They should NOT be equal
        assert nd6_values != nd3_values, \
            "ND6 and ND3 should produce different result sets"
