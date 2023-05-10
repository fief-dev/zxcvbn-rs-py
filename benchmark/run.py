import csv
import timeit

LIBRARIES = [
    "zxcvbn",
    "zxcvbn_rs_py",
]

PASSWORDS = {
    "Length 4": "igrI",
    "Length 8": "UzA170rl",
    "Length 16": "pvmXR8ECp10UIRCs",
    "Length 32": "MtvEJJUM2K0mKAyWKy0emf6Bt9xyxrru",
    "Repetition": "a" * 16,
}

NB_LOOP = 500
REPEAT = 5

with open("benchmark/benchmark.dat", "w") as file:
    writer = csv.writer(file, delimiter="\t", quoting=csv.QUOTE_NONNUMERIC)
    writer.writerow(("Case", *LIBRARIES))
    for key, value in PASSWORDS.items():
        row = [key]
        print(f"Run benchmark for {key}...")
        for library in LIBRARIES:
            result = min(
                timeit.repeat(
                    f"zxcvbn('{value}')",
                    setup=f"from {library} import zxcvbn",
                    number=NB_LOOP,
                    repeat=REPEAT,
                )
            )
            row.append(result / NB_LOOP * 1000)
        writer.writerow(row)
