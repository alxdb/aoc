module Main where

import Aoc.CLI

main :: IO ()
main = do
  (Options year day command) <- parseOptions
  case command of
    Init -> return ()
    Solve -> return ()
