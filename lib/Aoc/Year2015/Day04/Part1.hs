module Aoc.Year2015.Day04.Part1 (
    solution,
) where

import Control.Arrow
import Crypto.Hash.MD5 qualified as MD5
import Data.ByteString.Builder qualified as BSB
import Data.ByteString.Char8 qualified as BSC
import Data.ByteString.Lazy qualified as BSL
import Data.ByteString.Lazy.UTF8 qualified as BSLU

solution :: String -> Either String Int
solution s = Right $ head $ filter (\i -> hashPrefix (s <> show i) == "00000") [0 ..]

hashPrefix :: String -> String
hashPrefix =
    BSC.pack
        >>> MD5.hash
        >>> BSB.byteStringHex
        >>> BSB.toLazyByteString
        >>> BSL.take 5
        >>> BSLU.toString
