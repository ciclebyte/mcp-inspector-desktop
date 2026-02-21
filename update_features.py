import json

# Read the feature list
with open('feature_list.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

# MVP features that are implemented (F-001 through F-010)
completed = ['F-001', 'F-002', 'F-003', 'F-004', 'F-005', 'F-006', 'F-007', 'F-008', 'F-009', 'F-010']

# Update passes status
for feature in data['features']:
    if feature['id'] in completed:
        feature['passes'] = True

# Write back
with open('feature_list.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2, ensure_ascii=False)

print(f"Updated {len(completed)} features to passing status")
