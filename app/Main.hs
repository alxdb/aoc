module Main where

import Aoc.CLI

main :: IO ()
main = do
  (Options aocId command) <- parseOptions
  case command of
    Init -> return ()
    Solve -> return ()
