module Aoc.Year2015.Day02.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day02.Part1
import Test.Hspec

spec :: Spec
spec = describe "solution" $ do
  context "solves the examples" $
    testCases
      solution
      [ makeCase "2x3x4" 58
      , makeCase "1x1x10" 43
      ]
