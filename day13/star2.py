import fileinput
import re

from sympy import solve, symbols, nsolve
from dataclasses import dataclass

@dataclass
class XY:
  x: int
  y: int


class Solution:
  def __init__(self, buttons, prizes):
    self.buttons = buttons
    self.prizes = prizes

  def solve(self):
    n, m, = symbols('n m', positive=True, integer=True)
    cost = 0
    for i in range(len(self.buttons)):
      but_a, but_b = self.buttons[i]
      prize = self.prizes[i]
      res = solve(
        [
          but_a.x * n + but_b.x * m - prize.x,
          but_a.y * n + but_b.y * m - prize.y
        ],
        [n, m],
        dict=True,
      )
      #print(f"buttons: {(but_a, but_b)}, prize: {prize}, res: {res}")
      if res:
        cost += res[0][n] * 3 + res[0][m]
    return cost


BUT_RE = re.compile(r"Button [AB]: X\+(\d+), Y\+(\d+)")
PRI_RE = re.compile(r"Prize: X=(\d+), Y=(\d+)")
ADDITIVE = 10000000000000
#ADDITIVE = 0

def solution_from_stdin():
  inp = fileinput.input()
  buttons = []
  prizes = []
  for line in inp:
    b1_line = line.strip()
    b2_line = inp.readline().strip()
    pri_line = inp.readline().strip()
    inp.readline()  # empty line

    
    m1 = BUT_RE.match(b1_line)
    b_a = XY(int(m1.group(1)), int(m1.group(2)))

    m2 = BUT_RE.match(b2_line)
    b_b = XY(int(m2.group(1)), int(m2.group(2)))
    buttons.append((b_a, b_b))

    m3 = PRI_RE.match(pri_line)
    prizes.append(
      XY(
        ADDITIVE + int(m3.group(1)),
        ADDITIVE + int(m3.group(2))
      ))

  return Solution(buttons, prizes)

def main():
  sol = solution_from_stdin()
  print(sol.solve())

if __name__ == "__main__":
   main()
