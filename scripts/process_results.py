#!/usr/bin/env python3

import json
import sys
from collections import defaultdict

def load_results(filepath):
    with open(filepath, 'r') as f:
        return json.load(f)

def components_to_suffix(components):
    """Convert components list to a suffix string, e.g. [['x', 10]] -> '_x_10'"""
    if not components:
        return ''
    parts = []
    for comp in components:
        name, value = comp[0], comp[1]
        parts.append(f'_{name}_{value}')
    return ''.join(parts)

def components_key(components):
    """Create a hashable key from components."""
    if not components:
        return ()
    return tuple((c[0], c[1]) for c in components)

def process_results(data):
    # Group by pallet
    pallets = defaultdict(dict)

    for entry in data:
        pallet = entry['pallet']
        benchmark = entry['benchmark']

        # Group time_results by components
        by_components = defaultdict(list)
        for tr in entry['time_results']:
            key = components_key(tr['components'])
            by_components[key].append(tr['extrinsic_time'])

        # Create separate benchmark entries for each component combination
        for comp_key, times in by_components.items():
            suffix = ''
            if comp_key:
                suffix = ''.join(f'_{name}_{value}' for name, value in comp_key)
            full_name = benchmark + suffix
            avg_time = sum(times) / len(times)
            pallets[pallet][full_name] = avg_time

    return pallets

def pair_benchmarks(benchmarks):
    """Pair ark_ and sub_ benchmarks by their base name."""
    pairs = {}

    for name, time in benchmarks.items():
        if name.startswith('ark_'):
            base = name[4:]  # Remove 'ark_' prefix
            if base not in pairs:
                pairs[base] = {'ark': None, 'sub': None}
            pairs[base]['ark'] = time
        elif name.startswith('sub_'):
            base = name[4:]  # Remove 'sub_' prefix
            if base not in pairs:
                pairs[base] = {'ark': None, 'sub': None}
            pairs[base]['sub'] = time

    return pairs

def format_time(ns):
    """Format nanoseconds to a readable string."""
    if ns is None:
        return 'N/A'
    if ns >= 1_000_000_000:
        return f'{ns / 1_000_000_000:.2f} s'
    elif ns >= 1_000_000:
        return f'{ns / 1_000_000:.2f} ms'
    elif ns >= 1_000:
        return f'{ns / 1_000:.2f} us'
    else:
        return f'{ns:.2f} ns'

def compute_speedup(ark_time, sub_time):
    """Compute speedup ratio (sub/ark). >1 means arkworks is faster."""
    if ark_time is None or sub_time is None or ark_time == 0:
        return 'N/A'
    ratio = ark_time / sub_time
    return f'{ratio:.2f}x'

def generate_markdown(pallets):
    output = []

    for pallet, benchmarks in sorted(pallets.items()):
        pairs = pair_benchmarks(benchmarks)

        if not pairs:
            continue

        output.append(f'# {pallet}\n')
        output.append('| extrinsic | arkworks | substrate | speedup |')
        output.append('|-----------|----------|-----------|---------|')

        for base_name, times in sorted(pairs.items()):
            ark_time = times['ark']
            sub_time = times['sub']

            ark_str = format_time(ark_time)
            sub_str = format_time(sub_time)
            speedup = compute_speedup(ark_time, sub_time)

            output.append(f'| {base_name} | {ark_str} | {sub_str} | {speedup} |')

        output.append('')

    return '\n'.join(output)

def main():
    if len(sys.argv) != 2:
        print('Error: missing input file', file=sys.stderr)
        print(f'Usage: {sys.argv[0]} <results.json>', file=sys.stderr)
        sys.exit(1)

    input_file = sys.argv[1]
    data = load_results(input_file)
    pallets = process_results(data)
    markdown = generate_markdown(pallets)

    with open('results.md', 'w') as f:
        f.write(markdown)

    print('Generated results.md')

if __name__ == '__main__':
    main()
