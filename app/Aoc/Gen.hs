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
aocModuleName (AocId y d p) = printf "Aoc.Year%d.Day%02d.Part%d" y d p

solutionType :: HsType'
solutionType = var "String" --> var "Either" @@ var "String" @@ var "Int"

mapModule :: [AocId] -> HsModule'
mapModule aocIds =
  module'
    (Just "Aoc.Map")
    Nothing
    ( import' "Aoc.CLI"
        : [qualified' . import' $ fromString (aocModuleName aocId) | aocId <- aocIds]
    )
    [ typeSig "getSolution" $ var "AocId" --> var "Maybe" @@ solutionType
    , funBinds
        "getSolution"
        $ [ let
              y = bvar . fromString . show $ year aocId
              d = bvar . fromString . printf "%02d" $ day aocId
              p = bvar . fromString . show $ part aocId
              s = var . fromString $ aocModuleName aocId <> ".solution"
             in
              match [conP "AocId" [y, d, p]] (var "Just" @@ s)
          | aocId <- aocIds
          ]
          ++ [match [wildP] (var "Nothing")]
    ]

solutionModule :: AocId -> HsModule'
solutionModule aocId =
  module'
    (Just $ fromString . aocModuleName $ aocId)
    (Just [var "solution"])
    []
    [ typeSig "solution" solutionType
    , funBind "solution" $ match [] (var "undefined")
    ]

testModule :: AocId -> HsModule'
testModule aocId =
  module'
    (Just . fromString $ aocModuleName aocId <> "Spec")
    (Just [var "spec"])
    [ import' $ fromString (aocModuleName aocId)
    , import' "Test.Hspec"
    ]
    [ typeSig "spec" $ var "Spec"
    , funBind "spec" $ match [] (var "describe" @@ string "solution" @@ var "undefined")
    ]

genModule :: HsModule' -> IO String
genModule hsmod =
  runGhc (Just libdir) $
    getSessionDynFlags <&> showPpr `flip` hsmod
