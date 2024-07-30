module Aoc.Year2015.Day01.Part2 (
  solution,
) where

import Data.Functor
import Text.Printf

data State = Result Int | NoResult Int Int String | Error String

solution :: String -> Either String Int
solution = g . f . NoResult 1 0
 where
  f :: State -> State
  f (NoResult i l ('(' : s)) = f' i (l + 1) s
  f (NoResult i l (')' : s)) = f' i (l - 1) s
  f (NoResult _ _ (c : _)) = Error $ "unexpected char: " $> c
  f (NoResult _ _ []) = Error "unexpected end of input"
  f x = x

  f' :: Int -> Int -> String -> State
  f' i l s
    | l == (-1) = Result i
    | otherwise = f $ NoResult (i + 1) l s

  g :: State -> Either String Int
  g (Result x) = Right x
  g (Error s) = Left s
  g (NoResult _ l []) = Left "Never reached the basement, only reached level: " $> l
  g (NoResult i l s) = Left $ printf "Logic error: i=%d l=%d s=%s" i l s
