import json
import sys

print("loading file")
with open(sys.argv[1]) as f:
    data = json.load(f)
print("loaded")

latencies = []
for log in data:
    event = log["event"]
    if event["type"] == "http":
        latencies.append(event["latency_secs"])


latencies.sort()
llen = len(latencies)
mean = sum(latencies) / llen
median = latencies[llen // 2]
nnpct = latencies[(llen // 100) * 99]

print(f"{mean} {median} {nnpct}")
