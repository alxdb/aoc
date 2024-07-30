module Aoc.Year2015.Day02.Part2 (
  solution,
) where

import Aoc.Year2015.Day02.Part1 hiding (solution)

solution :: String -> Either String Int
solution = fmap (sum . map f) . parsePresents
 where
  f (Present l w h) = l
