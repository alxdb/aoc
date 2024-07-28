module Main where

import Aoc.CLI
import Aoc.Gen
import Aoc.Input
import GHC.Utils.Monad
import Paths_aoc
import System.Directory
import Text.Printf

inputDataFileName :: AocId -> FilePath
inputDataFileName (AocId y d _) = printf "data/aoc_year%d_day%02d" y d

solutionModuleFileName :: AocId -> FilePath
solutionModuleFileName (AocId y d p) = printf "lib/Aoc/Year%d/Day%02d/Part%d.hs" y d p

testModuleFileName :: AocId -> FilePath
testModuleFileName (AocId y d p) = printf "test/Aoc/Year%d/Day%02d/Part%dSpec.hs" y d p

main :: IO ()
main = do
  (Options aocId command) <- parseOptions
  case command of
    Init -> do
      unlessM (doesFileExist "aoc.cabal") $ do
        fail "Please only run from the root of the git repository"
      let inputFileName = inputDataFileName aocId
      unlessM (doesFileExist inputFileName) $ do
        inputContents <- getInput aocId
        writeFile inputFileName inputContents
    Solve -> return ()
