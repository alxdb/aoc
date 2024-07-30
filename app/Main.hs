module Main (main) where

import Aoc.CLI
import Aoc.Init
import Aoc.Map

main :: IO ()
main = do
  (Options aocId command) <- parseOptions
  case command of
    Init -> runInit aocId
    Solve -> case getSolution aocId of
      Just solution -> do
        input <- readFile $ inputDataFileName aocId
        case solution input of
          Right answer -> print answer
          Left errorMsg -> error errorMsg
      Nothing -> error "not yet solved, please initialize"
