#!/usr/bin/python3

import argparse
import json
import os
import requests
import shutil
import subprocess

YEAR = '2024'

parser = argparse.ArgumentParser(
  prog='NewDayCreator',
  description='Create new day for advent of code')
parser.add_argument('day_number', type=int,
                    help='The number of the day to create.')
parser.add_argument('-td', '--templates_dir', type=str,
                    default='templates',
                    help='Directory with templates to copy over.')
parser.add_argument('--input_url', type=str,
                    default='https://adventofcode.com/{year}/day/{day_number}/input',
                    help='Template of the URL to download input from')
parser.add_argument('--secrets', type=str,
                    default='secrets.json',
                    help='JSON file with secrets, eg. for login')


MAIN_FILE = 'main.rs'
SOLUTION_FILE = 'solution.rs'
DEPS_FILE = 'dependencies.toml'
CARGO_TOML_FILE = 'Cargo.toml'


def cargo_init(day_name: str, templates_dir: str):
  subprocess.run(['cargo', 'new', day_name, '--bin'], check=True)
  src_path = os.path.join(day_name, 'src')

  main_template_path = os.path.join(templates_dir, MAIN_FILE)
  main_path = os.path.join(src_path, MAIN_FILE)

  solution_template_path = os.path.join(templates_dir, SOLUTION_FILE)
  solution_path = os.path.join(src_path, SOLUTION_FILE)

  shutil.copyfile(main_template_path, main_path)
  shutil.copyfile(solution_template_path, solution_path)

  deps_path = os.path.join(templates_dir, DEPS_FILE)
  cargo_toml_path = os.path.join(src_path, CARGO_TOML_FILE)

  with open(deps_path, 'r') as df:
    deps_content = df.read()

  with open(cargo_toml_path, 'a') as mf:
    mf.write(deps_content)


def parse_secrets(secrets_file: str):
  with open(secrets_file) as f:
    return json.load(f)


def download_input(
    website_tmpl: str, session_token: str, day_num: int, dest_dir: str):
  dest_path = os.path.join(dest_dir, 'input')

  failed = False
  try:
    resp = requests.get(
      website_tmpl.format(year=YEAR, day_number=day_num),
      cookies={'session': session_token},
      headers={
        'User-Agent':
        f'https://github.com/anula/advent_of_code_{YEAR} - auto-get input'})
  except requests.ConnectionError as err:
    failed = True
    reason = err

  if not resp.ok:
    failed = True
    reason = resp.reason

  if failed:
    print(f'Failed to download input, reason: {reason}')

  with open(dest_path, 'wb') as f:
    f.write(resp.content)


def main():
  args = parser.parse_args()

  day_name = f'day{args.day_number}'

  if os.path.isdir(day_name):
    print(f'Directory "{day_name}" already exists, aborting...')
    return

  cargo_init(day_name, args.templates_dir)

  secrets = parse_secrets(args.secrets)
  download_input(
    args.input_url, secrets['aoc_session'], args.day_number, day_name)


if __name__ == "__main__":
  main()
