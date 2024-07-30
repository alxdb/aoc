module Aoc.Map where

import Aoc.CLI
import Aoc.Year2015.Day01.Part1 qualified
import Aoc.Year2015.Day01.Part2 qualified
import Aoc.Year2015.Day02.Part1 qualified
import Aoc.Year2015.Day02.Part2 qualified

getSolution :: AocId -> Maybe (String -> Either String Int)
getSolution (AocId 2015 02 2) =
  Just Aoc.Year2015.Day02.Part2.solution
getSolution (AocId 2015 02 1) =
  Just Aoc.Year2015.Day02.Part1.solution
getSolution (AocId 2015 01 2) =
  Just Aoc.Year2015.Day01.Part2.solution
getSolution (AocId 2015 01 1) =
  Just Aoc.Year2015.Day01.Part1.solution
getSolution _ = Nothing
