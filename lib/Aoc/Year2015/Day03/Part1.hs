module Aoc.Year2015.Day03.Part1 (
  solution,
  readDir,
  moveSanta,
) where

import Data.Bifunctor
import Data.Functor
import Data.Set qualified as Set

data Direction = N | E | S | W

readDir :: Char -> Either String Direction
readDir '^' = Right N
readDir 'v' = Right S
readDir '>' = Right E
readDir '<' = Right W
readDir c = Left $ "unexpected char: " $> c

moveSanta :: Direction -> (Int, Int) -> (Int, Int)
moveSanta N = second succ
moveSanta S = second pred
moveSanta E = first succ
moveSanta W = first pred

solution :: String -> Either String Int
solution = fmap f . mapM readDir
 where
  f = Set.size . snd . foldl g ((0, 0), Set.singleton (0, 0))
  g (lastHouse, visited) direction =
    let nextHouse = moveSanta direction lastHouse
     in (nextHouse, Set.insert nextHouse visited)
