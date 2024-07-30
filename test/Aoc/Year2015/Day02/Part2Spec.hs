module Aoc.Year2015.Day02.Part2Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day02.Part2
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase "2x3x4" 10
      , makeCase "1x1x10" 14
      ]
