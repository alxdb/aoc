module Aoc.Year2015.Day01.Part1 (
  solution,
) where

import Control.Monad
import Data.Functor

solution :: String -> Either String Int
solution = foldM f 0
  where
    f :: Int -> Char -> Either String Int
    f x '(' = Right (x + 1)
    f x ')' = Right (x - 1)
    f _ c = Left $ "unexpected char: " $> c
