module Aoc.Year2015.Day03.Part2Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day03.Part2
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase "^v" 3
      , makeCase "^>v<" 3
      , makeCase "^v^v^v^v^v" 11
      ]
