module Aoc.Year2015.Day01.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day01.Part1
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase "(())" 0
      , makeCase "(((" 3
      , makeCase "(()(()(" 3
      , makeCase "))(((((" 3
      , makeCase "())" (-1)
      , makeCase "))(" (-1)
      , makeCase ")))" (-3)
      , makeCase ")())())" (-3)
      ]
