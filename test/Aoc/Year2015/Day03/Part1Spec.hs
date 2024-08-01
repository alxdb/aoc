module Aoc.Year2015.Day03.Part1Spec (
  spec,
) where

import Aoc.TestCases
import Aoc.Year2015.Day03.Part1
import Test.Hspec

spec :: Spec
spec = describe "solution" $ testCasesSpec solution []
