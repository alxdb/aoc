module Aoc.CLI where

import Options.Applicative

data Options = Options
  { aocId :: AocId
  , aocCommand :: Command
  }

data AocId = AocId {year :: Int, day :: Int}
data Command = Init | Solve deriving (Read, Show, Eq)

makeOptions :: Int -> Int -> Command -> Options
makeOptions year day = Options (AocId year day)

options :: Parser Options
options =
  makeOptions
    <$> argument auto (metavar "YEAR")
    <*> argument auto (metavar "DAY")
    <*> hsubparser (initCommand <> solveCommand)
 where
  initCommand =
    command "init" $
      info (pure Init) (progDesc "Initialize the puzzle")
  solveCommand =
    command "solve" $
      info (pure Solve) (progDesc "Solve the puzzle (must be initialized)")

parseOptions :: IO Options
parseOptions =
  execParser $
    info (options <**> helper) $
      fullDesc
        <> progDesc "Solve advent of code problems"
        <> header "aoc - an advent of code solver"
