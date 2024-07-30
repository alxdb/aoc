module Aoc.Year2015.Day01.Part2Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day01.Part2
import Test.Hspec

spec :: Spec
spec = describe "solution" $ do
  context "solves the examples" $
    testCases
      solution
      [ makeCase ")" 1
      , makeCase "()())" 5
      ]
