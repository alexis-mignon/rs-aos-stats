"""
Property-based testing for dice bindings using Hypothesis.

This finds edge cases and verifies invariants hold for all inputs.
"""

from hypothesis import given, strategies as st
import rs_aos_stats as rs


class TestDiceProperties:
    """Property-based tests that should hold for ALL dice rolls."""

    @given(st.integers(min_value=1, max_value=6))
    def test_nd6_probabilities_always_sum_to_one(self, n):
        """For ANY number of D6 dice, probabilities must sum to 1."""
        dice = rs.ND6(n)
        results = dice.values_and_probas()

        total_prob = sum(p for v, p in results)

        assert abs(total_prob - 1.0) < 1e-9, \
            f"ND6({n}) probabilities sum to {total_prob}, not 1.0"

    @given(st.integers(min_value=1, max_value=10))
    def test_nd3_probabilities_always_sum_to_one(self, n):
        """For ANY number of D3 dice, probabilities must sum to 1."""
        dice = rs.ND3(n)
        results = dice.values_and_probas()

        total_prob = sum(p for v, p in results)

        assert abs(total_prob - 1.0) < 1e-9, \
            f"ND3({n}) probabilities sum to {total_prob}, not 1.0"

    @given(
        st.integers(min_value=1, max_value=6),
        st.integers(min_value=0, max_value=10)
    )
    def test_nd6_plus_range_property(self, n, bonus):
        """ND6(n) + bonus should have range [n+bonus, 6*n+bonus]."""
        dice = rs.ND6Plus(n, bonus)
        results = dice.values_and_probas()

        values = [v for v, p in results]

        expected_min = n + bonus
        expected_max = 6 * n + bonus

        assert min(values) == expected_min, \
            f"ND6Plus({n}, {bonus}) min should be {expected_min}"
        assert max(values) == expected_max, \
            f"ND6Plus({n}, {bonus}) max should be {expected_max}"

    @given(
        st.integers(min_value=1, max_value=6),
        st.integers(min_value=0, max_value=10)
    )
    def test_nd3_plus_range_property(self, n, bonus):
        """ND3(n) + bonus should have range [n+bonus, 3*n+bonus]."""
        dice = rs.ND3Plus(n, bonus)
        results = dice.values_and_probas()

        values = [v for v, p in results]

        expected_min = n + bonus
        expected_max = 3 * n + bonus

        assert min(values) == expected_min, \
            f"ND3Plus({n}, {bonus}) min should be {expected_min}"
        assert max(values) == expected_max, \
            f"ND3Plus({n}, {bonus}) max should be {expected_max}"

    @given(st.integers(min_value=1, max_value=6))
    def test_nd6_expected_value_property(self, n):
        """Expected value of ND6 should be n * 3.5."""
        dice = rs.ND6(n)
        results = dice.values_and_probas()

        expected_value = sum(v * p for v, p in results)
        theoretical_ev = n * 3.5

        assert abs(expected_value - theoretical_ev) < 1e-6, \
            f"ND6({n}) expected value should be {theoretical_ev}, got {expected_value}"

    @given(st.integers(min_value=1, max_value=10))
    def test_nd3_expected_value_property(self, n):
        """Expected value of ND3 should be n * 2.0."""
        dice = rs.ND3(n)
        results = dice.values_and_probas()

        expected_value = sum(v * p for v, p in results)
        theoretical_ev = n * 2.0

        assert abs(expected_value - theoretical_ev) < 1e-6, \
            f"ND3({n}) expected value should be {theoretical_ev}, got {expected_value}"
