module Aoc.Parser (Parser, parse) where

import Data.Bifunctor
import Data.Void
import Text.Megaparsec hiding (parse)
import Text.Megaparsec qualified as MP (parse)

type Parser = Parsec Void String

parse :: Parser a -> String -> Either String a
parse p = first errorBundlePretty . MP.parse p ""
