module Main (main) where

import Aoc.CLI
import Aoc.Init

main :: IO ()
main = do
  (Options aocId command) <- parseOptions
  case command of
    Init -> runInit aocId
    Solve -> return ()
