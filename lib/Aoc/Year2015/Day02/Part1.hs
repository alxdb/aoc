module Aoc.Year2015.Day02.Part1 (
  solution,
  Present (..),
  parsePresents,
) where

import Aoc.Parser
import Text.Megaparsec hiding (parse)
import Text.Megaparsec.Char
import Text.Megaparsec.Char.Lexer

data Present = Present Int Int Int

wrappingPaper :: Present -> Int
wrappingPaper (Present l w h) = (2 * a) + (2 * b) + (2 * c) + minimum [a, b, c]
 where
  a = l * w
  b = w * h
  c = h * l

presentP :: Parser Present
presentP = do
  [l, w, h] <- decimal `sepBy` char 'x'
  return $ Present l w h

parsePresents :: String -> Either String [Present]
parsePresents = parse (presentP `sepBy` char '\n')

solution :: String -> Either String Int
solution = fmap (sum . map wrappingPaper) . parsePresents
