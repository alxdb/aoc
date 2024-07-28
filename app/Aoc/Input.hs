{-# LANGUAGE OverloadedStrings #-}

module Aoc.Input (getInput) where

import Data.ByteString.UTF8
import Data.Function
import Data.Functor
import Network.HTTP.Simple
import System.Environment
import Text.Printf

formatUrl :: Int -> Int -> String
formatUrl = printf "https://adventofcode.com/%d/day/%d/input"

aocTokenVar :: String
aocTokenVar = "AOC_TOKEN"

missingTokenErrorMessage :: String
missingTokenErrorMessage =
  "Please provide the session token via the `" <> aocTokenVar <> "` environment variable"

getToken :: IO String
getToken = lookupEnv aocTokenVar >>= maybe (fail missingTokenErrorMessage) return

getInput :: Int -> Int -> IO String
getInput year day = do
  let url = formatUrl year day
  token <- getToken <&> fromString
  req <- parseRequest url <&> addRequestHeader "cookie" token
  res <- httpBS req <&> getResponseBody
  return $ res & toString
