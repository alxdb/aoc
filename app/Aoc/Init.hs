module Aoc.Init (runInit) where

import Aoc.CLI
import Aoc.Gen
import Aoc.Input
import Control.Monad.Extra
import Data.Char
import Data.Functor
import Data.Maybe
import System.Directory
import System.FilePath
import System.Process
import Text.ParserCombinators.ReadP
import Text.Printf

runInit :: AocId -> IO ()
runInit aocId = do
  unlessM (doesFileExist "aoc.cabal") $ fail "Please only run from the root of the project"
  writeFileUnlessExists (inputDataFileName aocId) (getInput aocId)
  writeFileUnlessExists (solutionModuleFileName aocId) (genModule $ solutionModule aocId)
  writeFileUnlessExists (testModuleFileName aocId) (genModule $ testModule aocId)
  getCurrentSolutions
    >>= genModule . mapModule
    >>= writeFile "app/Aoc/Map.hs"
  callCommand "hpack"
  forM_ [solutionModuleFileName aocId, testModuleFileName aocId, "app/Aoc/Map.hs"] $ \moduleFileName ->
    callCommand $ "fourmolu -i " <> moduleFileName

getCurrentSolutions :: IO [AocId]
getCurrentSolutions = do
  filepaths <- recursiveDirectoryList "lib"
  return $ mapMaybe parseAocIdFromFileName filepaths

parseAocIdFromFileName :: FilePath -> Maybe AocId
parseAocIdFromFileName filepath = fst <$> (listToMaybe . readP_to_S parser $ filepath)
 where
  parser :: ReadP AocId
  parser = do
    _ <- string "lib/Aoc/Year"
    year <- munch isDigit <&> read
    _ <- string "/Day"
    day <- munch isDigit <&> read
    _ <- string "/Part"
    part <- munch isDigit <&> read
    _ <- string ".hs"
    return $ AocId year day part

recursiveDirectoryList :: FilePath -> IO [FilePath]
recursiveDirectoryList dir = do
  contents <- map (dir </>) <$> listDirectory dir
  files <- filterM doesFileExist contents
  subdirs <- filterM doesDirectoryExist contents
  subdirsContent <- concatMapM recursiveDirectoryList subdirs
  return $ files ++ subdirsContent

inputDataFileName :: AocId -> FilePath
inputDataFileName (AocId y d _) = printf "data/aoc_year%d_day%02d" y d

solutionModuleFileName :: AocId -> FilePath
solutionModuleFileName (AocId y d p) = printf "lib/Aoc/Year%d/Day%02d/Part%d.hs" y d p

testModuleFileName :: AocId -> FilePath
testModuleFileName (AocId y d p) = printf "test/Aoc/Year%d/Day%02d/Part%dSpec.hs" y d p

writeFileUnlessExists :: FilePath -> IO String -> IO ()
writeFileUnlessExists filePath genContents =
  unlessM (doesFileExist filePath) $ do
    contents <- genContents
    createDirectoryIfMissing True (takeDirectory filePath)
    writeFile filePath contents
