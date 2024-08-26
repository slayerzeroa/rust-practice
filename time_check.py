import pandas as pd
import numpy as np


df1 = pd.read_csv('trade_data_python.csv')
df2 = pd.read_csv('trade_data_rust.csv')

df2['trade_timestamp'] = df2['trade_timestamp'].astype(np.int64)


# trade_timestamp 열 기준으로 df1, df2 병합
df = pd.merge_asof(df1, df2, on='trade_timestamp', direction='nearest')

df.columns = ['trade_timestamp', 'elapsed_time_python', 'elapsed_time_rust']

df['diff'] = df['elapsed_time_python'] - df['elapsed_time_rust']
df['winning'] = df['diff'] > 0

python_win = len(df[df['winning'] == False])
rust_win = len(df[df['winning'] == True])

python_rate = python_win / (python_win + rust_win) * 100
rust_rate = 100 - python_rate


python_win_diff = df[df['winning'] == False]['diff'].apply(lambda x: abs(x)).mean()
rust_win_diff = df[df['winning'] == True]['diff'].apply(lambda x: abs(x)).mean()

print(f"Python Winning Times: {len(df[df['winning'] == False])}")
print(f"Rust Winning Times: {len(df[df['winning'] == True])}")

print(f"Python Winning Rates: {python_rate:.2f}%")
print(f"Rust Winning Rates: {rust_rate:.2f}%")


print(f"When Python Win: {python_win_diff:.10f}")
print(f"When Rust Win: {rust_win_diff:.10f}")