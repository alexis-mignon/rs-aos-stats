#!/bin/bash

# Manual API Testing Script
# Runs various curl commands to test the API

API_URL="http://localhost:8001"

echo "🧪 API Manual Testing"
echo "===================="
echo ""

# Check if API is running
echo "1. Health Check..."
response=$(curl -s "$API_URL/health")
if echo "$response" | grep -q "ok"; then
    echo "   ✅ API is healthy: $response"
else
    echo "   ❌ API health check failed"
    exit 1
fi
echo ""

# Test basic calculation
echo "2. Basic Calculation (10 attacks, fixed values)..."
result=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "10",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "1",
    "save": 4,
    "ward": null,
    "hit_rule_type": "normal"
  }' | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"Mean: {r['mean_damage']:.4f}, Max: {r['max_damage']}, Points: {len(r['probabilities'])}\")")
echo "   Result: $result"
echo ""

# Test dice notation
echo "3. Dice Notation (D6 attacks, D3 damage)..."
result=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "D6",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "D3",
    "save": 4,
    "ward": null,
    "hit_rule_type": "normal"
  }' | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"Mean: {r['mean_damage']:.4f}, Max: {r['max_damage']}\")")
echo "   Result: $result"
echo ""

# Test compound dice
echo "4. Compound Dice (2D6+3 attacks, 2D6 damage)..."
result=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "2D6+3",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "2D6",
    "save": 4,
    "ward": null,
    "hit_rule_type": "normal"
  }' | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"Mean: {r['mean_damage']:.4f}, Max: {r['max_damage']}\")")
echo "   Result: $result"
echo ""

# Test all hit rules
echo "5. All Hit Rules (should increase damage)..."
for rule in normal crit_auto_wound crit_mortal_wound crit_double_hit; do
    mean=$(curl -s -X POST "$API_URL/api/calculate" \
      -H "Content-Type: application/json" \
      -d "{
        \"attacks_characteristic\": \"10\",
        \"to_hit\": 3,
        \"to_wound\": 3,
        \"rend\": 1,
        \"damage_characteristic\": \"1\",
        \"save\": 4,
        \"ward\": null,
        \"hit_rule_type\": \"$rule\"
      }" | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"{r['mean_damage']:.4f}\")")
    printf "   %-20s: %s\n" "$rule" "$mean"
done
echo ""

# Test ward saves
echo "6. With Ward Save (should reduce damage)..."
no_ward=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "10",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "2",
    "save": 4,
    "ward": null,
    "hit_rule_type": "normal"
  }' | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"{r['mean_damage']:.4f}\")")

with_ward=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "10",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "2",
    "save": 4,
    "ward": 5,
    "hit_rule_type": "normal"
  }' | python3 -c "import sys, json; r=json.load(sys.stdin); print(f\"{r['mean_damage']:.4f}\")")

echo "   No ward:   $no_ward"
echo "   With 5+ ward: $with_ward"
echo ""

# Test error handling
echo "7. Error Handling (invalid hit rule)..."
response=$(curl -s -X POST "$API_URL/api/calculate" \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "10",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "1",
    "save": 4,
    "ward": null,
    "hit_rule_type": "invalid"
  }')
if echo "$response" | grep -q "detail"; then
    echo "   ✅ Error properly caught"
else
    echo "   ❌ Error not handled"
fi
echo ""

echo "✅ Manual tests complete!"
