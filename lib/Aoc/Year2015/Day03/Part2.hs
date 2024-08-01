module Aoc.Year2015.Day03.Part2 (
  solution,
) where

import Aoc.Year2015.Day03.Part1 hiding (solution)
import Data.Bifunctor
import Data.Set qualified as Set

data Santa = R | H

solution :: String -> Either String Int
solution = fmap f . mapM readDir
 where
  f = Set.size . snd . foldl g initialState . zip (cycle [R, H])
  g (houses, visited) (santa, direction) =
    let (getHouse, updateHouse) = h santa
        nextHouse = moveSanta direction (getHouse houses)
     in (updateHouse (const nextHouse) houses, Set.insert nextHouse visited)
  h R = (snd, second)
  h H = (fst, first)

  initialState = (((0, 0), (0, 0)), Set.singleton (0, 0))
