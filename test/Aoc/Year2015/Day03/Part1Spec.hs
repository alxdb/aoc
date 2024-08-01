module Aoc.Year2015.Day03.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day03.Part1
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase ">" 2
      , makeCase "^>v<" 4
      , makeCase "^v^v^v^v^v" 2
      ]
