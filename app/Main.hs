module Main (main) where

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

writeFileUnlessExists :: FilePath -> IO String -> IO ()
writeFileUnlessExists filePath genContents = do
  unlessM (doesFileExist filePath) $ do
    -- TODO: Create directory for path if missing
    contents <- genContents
    writeFile filePath contents

main :: IO ()
main = do
  (Options aocId command) <- parseOptions
  case command of
    Init -> do
      unlessM (doesFileExist "aoc.cabal") $ do
        fail "Please only run from the root of the project"
      writeFileUnlessExists (inputDataFileName aocId) (getInput aocId)
      writeFileUnlessExists (solutionModuleFileName aocId) (genModule $ solutionModule aocId)
      writeFileUnlessExists (testModuleFileName aocId) (genModule $ testModule aocId)
      aocModuleFiles <- listDirectory "lib/Aoc"
      print aocModuleFiles
    Solve -> return ()
