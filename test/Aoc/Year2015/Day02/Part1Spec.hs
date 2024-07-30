module Aoc.Year2015.Day02.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day02.Part1
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase "2x3x4" 58
      , makeCase "1x1x10" 43
      , makeCase "1x1x10\n2x3x4" (58 + 43)
      ]
