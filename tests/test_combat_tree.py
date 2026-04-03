import pytest
from rs_aos_stats import (
    AttackStats, DefenseStats, CombatConfig, RollModifier, compute_damages,
)


class TestComputeDamages:
    """Tests for the compute_damages end-to-end function."""

    def test_basic_sequence(self):
        """Standard sequence should return probabilities summing to 1."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "normal")

        assert len(result) > 0
        total = sum(p for _, p in result)
        assert abs(total - 1.0) < 1e-10

    def test_result_contains_zero_damage(self):
        """Results should include a zero-damage entry (possibility of all misses)."""
        attack_stats = AttackStats(1, 4, 4, 0, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "normal")

        damages = [d for d, _ in result]
        assert 0 in damages

    def test_nonzero_damage_probability(self):
        """With reasonable stats, some nonzero damage should be possible."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "normal")

        nonzero = [(d, p) for d, p in result if d > 0]
        assert len(nonzero) > 0

    def test_max_damage_bounded(self):
        """Max damage should not exceed attacks * damage_per_hit."""
        attacks = 3
        damage_per_hit = 2
        attack_stats = AttackStats(attacks, 3, 3, 1, damage_per_hit)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "normal")

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

        result_no_ward = compute_damages(config_no_ward, "normal")
        result_with_ward = compute_damages(config_with_ward, "normal")

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

        result_no_mod = compute_damages(config_no_mod, "normal")
        result_with_mod = compute_damages(config_with_mod, "normal")

        avg_no_mod = sum(d * p for d, p in result_no_mod)
        avg_with_mod = sum(d * p for d, p in result_with_mod)
        assert avg_with_mod > avg_no_mod

    def test_crit_double_hit_increases_max_damage(self):
        """CritDoubleHitRule should allow more max damage than attacks * damage."""
        attack_stats = AttackStats(2, 6, 2, 0, 1)  # 6+ to hit = crits only
        defense_stats = DefenseStats(7, None)  # No save
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "crit_double_hit")

        max_d = max(d for d, _ in result)
        # With 2 attacks and CritDoubleHitRule, max hits = 4 (each crit gives 2)
        assert max_d > 2

    def test_dice_attacks(self):
        """Using a dice string for attacks should produce valid results."""
        attack_stats = AttackStats("D6", 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        result = compute_damages(config, "normal")

        assert len(result) > 0
        total = sum(p for _, p in result)
        assert abs(total - 1.0) < 1e-10

    def test_invalid_hit_rule_type(self):
        """Passing an invalid hit_rule_type should raise."""
        attack_stats = AttackStats(3, 3, 3, 1, 1)
        defense_stats = DefenseStats(4, None)
        config = CombatConfig(attack_stats, defense_stats, None)

        with pytest.raises(BaseException):
            compute_damages(config, "invalid_rule")


def _mean(result):
    return sum(d * p for d, p in result)


class TestCritRulesIncreaseDamage:
    """Each crit rule should produce strictly higher mean damage than normal."""

    @pytest.fixture()
    def standard_config(self):
        return CombatConfig(
            AttackStats(10, 3, 3, 1, 1), DefenseStats(4, None), None,
        )

    @pytest.fixture()
    def normal_mean(self, standard_config):
        return _mean(compute_damages(standard_config, "normal"))

    @pytest.mark.parametrize("hit_rule_type", [
        "crit_auto_wound",
        "crit_mortal_wound",
        "crit_double_hit",
    ])
    def test_crit_increases_mean(self, standard_config, normal_mean, hit_rule_type):
        crit_mean = _mean(compute_damages(standard_config, hit_rule_type))
        assert crit_mean > normal_mean

    @pytest.mark.parametrize("hit_rule_type", [
        "crit_auto_wound",
        "crit_mortal_wound",
        "crit_double_hit",
    ])
    def test_crit_increases_mean_at_six_plus(self, hit_rule_type):
        """Crit benefit is most pronounced when only crits hit (6+)."""
        config = CombatConfig(
            AttackStats(10, 6, 3, 0, 1), DefenseStats(4, None), None,
        )
        normal = _mean(compute_damages(config, "normal"))
        crit = _mean(compute_damages(config, hit_rule_type))
        assert crit > normal

    @pytest.mark.parametrize("hit_rule_type", [
        "crit_auto_wound",
        "crit_mortal_wound",
        "crit_double_hit",
    ])
    def test_crit_increases_mean_with_ward(self, hit_rule_type):
        """Crit rules should still increase damage when a ward save is present."""
        config = CombatConfig(
            AttackStats(10, 3, 3, 1, 2), DefenseStats(4, 5), None,
        )
        normal = _mean(compute_damages(config, "normal"))
        crit = _mean(compute_damages(config, hit_rule_type))
        assert crit > normal


class TestWardReducesDamage:
    """Ward saves should strictly reduce mean damage."""

    @pytest.mark.parametrize("attacks,to_hit,to_wound,rend,dmg,save,ward", [
        (10, 3, 3, 1, 1, 4, 4),
        (5, 2, 2, 0, 2, 5, 5),
        (3, 4, 4, 2, 3, 3, 6),
    ])
    def test_ward_reduces_mean(self, attacks, to_hit, to_wound, rend, dmg, save, ward):
        config_ward = CombatConfig(
            AttackStats(attacks, to_hit, to_wound, rend, dmg),
            DefenseStats(save, ward), None,
        )
        config_no_ward = CombatConfig(
            AttackStats(attacks, to_hit, to_wound, rend, dmg),
            DefenseStats(save, None), None,
        )
        mean_with = _mean(compute_damages(config_ward, "normal"))
        mean_without = _mean(compute_damages(config_no_ward, "normal"))
        assert mean_with < mean_without

    def test_stronger_ward_reduces_more(self):
        """A 4+ ward should reduce damage more than a 5+ ward."""
        attack_stats = AttackStats(10, 3, 3, 1, 2)
        config_4plus = CombatConfig(attack_stats, DefenseStats(4, 4), None)
        config_5plus = CombatConfig(attack_stats, DefenseStats(4, 5), None)

        mean_4 = _mean(compute_damages(config_4plus, "normal"))
        mean_5 = _mean(compute_damages(config_5plus, "normal"))
        assert mean_4 < mean_5
