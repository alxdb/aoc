module Aoc.Map where

import Aoc.CLI
import Aoc.Year2015.Day01.Part1 qualified

getSolution :: AocId -> Maybe (String -> Either String Int)
getSolution (AocId 2015 01 1) =
  Just Aoc.Year2015.Day01.Part1.solution
getSolution _ = Nothing
