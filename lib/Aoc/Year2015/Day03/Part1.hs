module Aoc.Year2015.Day03.Part1 (
  solution,
) where

import Data.Bifunctor
import Data.Functor
import Data.Maybe
import Data.Set.Ordered (OSet, (|<))
import Data.Set.Ordered qualified as OSet

data Direction = N | E | S | W deriving (Eq)

readDir :: Char -> Either String Direction
readDir '^' = Right N
readDir '>' = Right E
readDir 'v' = Right S
readDir '<' = Right W
readDir c = Left $ "unexpected char: " $> c

lastVisited :: OSet (Int, Int) -> (Int, Int)
lastVisited = fromJust . flip OSet.elemAt 0

solution :: String -> Either String Int
solution = fmap (OSet.size . foldl f (OSet.singleton (0, 0))) . mapM readDir
 where
  f :: OSet (Int, Int) -> Direction -> OSet (Int, Int)
  f visited N = second succ (lastVisited visited) |< visited
  f visited E = first succ (lastVisited visited) |< visited
  f visited S = second pred (lastVisited visited) |< visited
  f visited W = first pred (lastVisited visited) |< visited
