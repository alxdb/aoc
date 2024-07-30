module Aoc.Year2015.Day01.Part1Spec (
  spec,
) where

import Aoc.Year2015.Day01.Part1
import Test.Hspec

spec :: Spec
spec = describe "solution" $ do
  context "solves the examples" $ do
    testCase "(())" 0
    testCase "(((" 3
    testCase "(()(()(" 3
    testCase "))(((((" 3
    testCase "())" (-1)
    testCase "))(" (-1)
    testCase ")))" (-3)
    testCase ")())())" (-3)
 where
  testCase i r = it ("given " <> i) $
    do solution i `shouldBe` Right r
