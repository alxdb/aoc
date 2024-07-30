module Aoc.TestCases (testCases, makeCase, nameCase) where

import Test.Hspec

data TestCase = TestCase (Maybe String) String Int

makeCase :: String -> Int -> TestCase
makeCase = TestCase Nothing

nameCase :: String -> String -> Int -> TestCase
nameCase name = TestCase (Just name)

testCases :: (String -> Either String Int) -> [TestCase] -> SpecWith ()
testCases solution = mapM_ f
  where
    f :: TestCase -> SpecWith (Arg Expectation)
    f (TestCase (Just name) input expected) = it name $ do solution input `shouldBe` Right expected
    f (TestCase Nothing input expected) = it input $ do solution input `shouldBe` Right expected
