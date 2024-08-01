module Aoc.Year2015.Day02.Part2 (
  solution,
) where

import Aoc.Year2015.Day02.Part1 hiding (solution)
import Data.Function
import Data.List

ribbonLength :: Present -> Int
ribbonLength (Present l w h) =
  sort [l, w, h]
    & take 2
    & concatMap (replicate 2)
    & sum

bowLength :: Present -> Int
bowLength (Present l w h) = l * w * h

solution :: String -> Either String Int
solution = fmap (sum . map f) . parsePresents
 where
  f p = ribbonLength p + bowLength p
