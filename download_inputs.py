
import io
import os
import sys
from time import sleep
import requests

try:
    with open('year.txt', 'r') as f:
        YEAR=f.read().strip()
except IOError as e:
    print("Error reading year.txt file. Doesn't exist?", e, file=sys.stderr)
    exit(1)

try:
    with open('session.txt', 'r') as f:
        SESSION=f.read().strip()
except IOError as e:
    print("Error reading session.txt file. Doesn't exist?", e, file=sys.stderr)
    exit(1)

for n in range(1, 32):
    st=os.stat(f'input/{str(n).zfill(2)}.txt')
    if st.st_size != 0:
        print(f'[+][Y{YEAR}D{str(n).zfill(2)}] Input already exists. Skipping.')

    print(f'[+][Y{YEAR}D{str(n).zfill(2)}] Downloading...')
    resp = requests.get(
        f'https://adventofcode.com/{YEAR}/day/{n}/input',
        cookies={
            'session': SESSION
        }
    )
    
    if resp.status_code == 400:
        print(f'[-] Error getting input for day {n} (status code {resp.status_code}). Expired session cookie?', file=sys.stderr)
        exit(1)
    elif resp.status_code == 404:
        print(f'[-] Error getting input for day {n} (status code {resp.status_code}). Day input not yet available?', file=sys.stderr)
        exit(1)
    elif resp.status_code != 200:
        print(f'[-] Error getting input for day {n} (status code {resp.status_code}). Expired session cookie or day input not yet available?', file=sys.stderr)
        continue

    with open(f'input/{str(n).zfill(2)}.txt', 'w') as f:
        f.write(resp.text)
    print(f'[+][Y{YEAR}D{str(n).zfill(2)}] Sucessfully downloaded. Waiting 10 secs for the next one...')
    sleep(10)


