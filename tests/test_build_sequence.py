#!/usr/bin/env python3
"""
Test the build_standard_sequence helper function in Python bindings.

This demonstrates the new convenience function that builds standard rule sequences,
matching the functionality provided by the WASM bindings.
"""

import rs_aos_stats as aos


def test_build_standard_sequence():
    """Test that build_standard_sequence creates correct rule sequences"""

    print("Testing build_standard_sequence helper function")
    print("=" * 70)

    # Test configuration
    attack_stats = aos.AttackStats(10, 3, 3, 1, 1)
    defense_stats = aos.DefenseStats(4, 5)  # 4+ save, 5+ ward
    config = aos.CombatConfig(attack_stats, defense_stats, None)

    test_cases = [
        ("normal", False),
        ("normal", True),
        ("crit_auto_wound", False),
        ("crit_mortal_wound", True),
        ("crit_double_hit", False),
    ]

    for hit_rule_type, has_ward in test_cases:
        # Build sequence using the helper
        sequence = aos.build_standard_sequence(hit_rule_type, has_ward)

        # Compute damages
        result = aos.compute_damages(config, sequence)
        mean = sum(d * p for d, p in result)

        # Verify sequence structure
        rule_types = [type(r).__name__ for r in sequence]
        expected_length = 6 if has_ward else 5

        print(f"\n{hit_rule_type:20s} (ward={has_ward})")
        print(f"  Rules: {' → '.join(rule_types)}")
        print(f"  Length: {len(sequence)} (expected {expected_length})")
        print(f"  Mean damage: {mean:.4f}")

        # Verify structure
        assert len(sequence) == expected_length, f"Expected {expected_length} rules, got {len(sequence)}"
        assert rule_types[0] == "AttackCharacteristicRule", "Should start with AttackCharacteristicRule"
        assert rule_types[-1] == "DamagesRule", "Should end with DamagesRule"
        if has_ward:
            assert "WardRule" in rule_types, "Should include WardRule when has_ward=True"

    # Test error handling
    print("\n" + "-" * 70)
    print("Testing error handling:")
    try:
        aos.build_standard_sequence("invalid_rule", False)
        print("  ❌ FAILED: Should have raised ValueError")
        return False
    except ValueError as e:
        print(f"  ✓ Correctly raised ValueError: {e}")

    print("\n" + "=" * 70)
    print("✅ All tests passed!")
    return True


if __name__ == "__main__":
    success = test_build_standard_sequence()
    exit(0 if success else 1)
