{-# LANGUAGE OverloadedStrings #-}

module Aoc.Gen where

import Aoc.CLI
import Data.Functor
import Data.String
import GHC
import GHC.Paths
import GHC.SourceGen
import Text.Printf

aocModuleName :: AocId -> String
aocModuleName (AocId y d) = printf "Aoc.Year%d.Day%02d" y d

mapModule :: [AocId] -> HsModule'
mapModule aocIds =
  module'
    (Just "Aoc.Map")
    (Just [var "getSolution"])
    ( import' "Aoc.CLI"
        : [qualified' . import' $ fromString (aocModuleName aocId) | aocId <- aocIds]
    )
    [ typeSig "getSolution" $ var "AocId" --> var "Maybe" @@ (var "String" --> var "Int")
    , funBinds
        "getSolution"
        $ [ let
              y = bvar . fromString . show $ year aocId
              d = bvar . fromString . printf "%02d" $ day aocId
              s = var . fromString $ aocModuleName aocId <> ".solution"
             in
              match [conP "AocId" [y, d]] (var "Just" @@ s)
          | aocId <- aocIds
          ]
          ++ [match [wildP] (var "Nothing")]
    ]

genModule :: [AocId] -> ([AocId] -> HsModule') -> IO String
genModule aocIds modGen =
  runGhc (Just libdir) $
    getSessionDynFlags <&> showPpr `flip` modGen aocIds
