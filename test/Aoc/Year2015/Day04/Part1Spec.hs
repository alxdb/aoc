module Aoc.Year2015.Day04.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day04.Part1
import Test.Hspec

spec :: Spec
spec =
  describe "solution" $
    testCasesSpec
      solution
      [ makeCase "abcdef" 609043
      , makeCase "pqrstuv" 1048970
      ]
