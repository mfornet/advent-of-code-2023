#!/usr/bin/env python3

import matplotlib.pyplot as plt


def time_as_ms(time: str):
    if time.endswith("µs"):
        time = float(time[:-2]) / 1000
    elif time.endswith("ms"):
        time = float(time[:-2])
    elif time.endswith("ns"):
        time = float(time[:-2]) / 1000000
    elif time.endswith("s"):
        time = float(time[:-1]) * 1000
    else:
        raise ValueError(f"Unknown time unit {time}")
    return time


def main():
    with open("README.md", "r") as f:
        readme = f.read()

    pattern = "<!--- benchmarking table --->"
    start = readme.find(pattern)
    end = readme.find(pattern, start + 1)

    table = readme[start:end]

    lines = [line for line in table.split("\n") if line.count("|") == 4][2:]

    stats = []

    for id, line in enumerate(lines):
        day = id + 1
        _, _, part1, part2, _ = line.split("|")
        part1 = time_as_ms(part1.strip(" `"))
        part2 = time_as_ms(part2.strip(" `"))
        stats.append((f"{day}-1", part1))
        stats.append((f"{day}-2", part2))

    # Create a pie chart with all times (part 1 and part 2 from each day)
    # Create a bar chart with the time for each day (part 1 and part 2)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5))

    values = [stat[1] for stat in stats]
    labels = [stat[0] for stat in stats]

    ax1.pie(
        values,
        labels=labels,
        autopct="%1.1f%%",
        startangle=90,
    )
    ax1.set_title("Relative time per part")

    ax2.bar(labels, values)
    ax2.set_ylabel("ms")
    ax2.set_xticklabels(labels, rotation=45, ha="right")
    ax2.set_title("Absolute time per part")

    plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    main()
