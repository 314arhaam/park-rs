import pandas as pd
import uuid, random, datetime, sys

if __name__ == '__main__':
    try: 
        filename = sys.argv[1]
        num = int(sys.argv[2])
    except IndexError:
        filename = "test.parquet"
        num = 10
    print(f"[*] generate data: `{filename}`, `{num}`")
    data = {
        "date_": [],
        "user_id": [],
        "user_name": [],
        "score": []
    }
    for i in range(num):
        data["date_"].append(
            datetime.datetime(
                year = 2025,
                month = random.randint(1, 12),
                day = random.randint(1, 28),
                hour = random.randint(0, 23),
                minute = random.randint(0, 59),
                second = random.randint(0, 59)
            ).strftime("%Y-%m-%d %H:%M:%S")
        )
        data["user_id"].append(str(uuid.uuid4()))
        tmp_name = ""
        for i in range(random.randint(5, 10)):
            tmp_name += chr(65+random.randint(0, 25))
        data["user_name"].append(tmp_name)
        data["score"].append(round(random.random()*100, 3))
    df = pd.DataFrame(data)
    print(df.head())
    print(df.shape)
    df.to_parquet(filename)