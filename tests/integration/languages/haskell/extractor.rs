use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_haskell_function_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
greet :: String -> String
greet name = "Hello, " ++ name ++ "!"

add :: Int -> Int -> Int
add x y = x + y

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)
"#,
    total_chunks: 8,
    chunk_counts: {
        Function: 7,
    }
}

test_language_extractor! {
    name: test_haskell_data_type_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
data Person = Person String Int

data Maybe a = Nothing | Just a

data Tree a = Leaf a | Node (Tree a) (Tree a)
"#,
    total_chunks: 4,
    chunk_counts: {
        Class: 3,
    }
}

test_language_extractor! {
    name: test_haskell_type_class_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
class Eq a where
  (==) :: a -> a -> Bool
  (/=) :: a -> a -> Bool

class Show a where
  show :: a -> String

instance Eq Bool where
  True == True = True
  False == False = True
  _ == _ = False
"#,
    total_chunks: 10,
    chunk_counts: {
        Function: 9,
    }
}

test_language_extractor! {
    name: test_haskell_module_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
module Math.Utils (
  add,
  multiply,
  square
) where

add :: Num a => a -> a -> a
add x y = x + y

multiply :: Num a => a -> a -> a
multiply x y = x * y

square :: Num a => a -> a
square x = x * x
"#,
    total_chunks: 8,
    chunk_counts: {
        Function: 6,
        Module: 1,
    }
}

test_language_extractor! {
    name: test_haskell_pattern_matching_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
head' :: [a] -> a
head' [] = error "Empty list"
head' (x:_) = x

length' :: [a] -> Int
length' [] = 0
length' (_:xs) = 1 + length' xs

map' :: (a -> b) -> [a] -> [b]
map' _ [] = []
map' f (x:xs) = f x : map' f xs
"#,
    total_chunks: 10,
    chunk_counts: {
        Function: 9,
    }
}

test_language_extractor! {
    name: test_haskell_comprehensive_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
module TestModule (
    Person(..),
    greet,
    calculate
) where

import Data.List (sort)
import qualified Data.Map as M

data Person = Person String Int deriving (Show, Eq)

data Tree a = Leaf a | Node (Tree a) (Tree a) deriving Show

class Drawable a where
    draw :: a -> String

instance Drawable Person where
    draw (Person name age) = name ++ " (" ++ show age ++ ")"

greet :: Person -> String
greet (Person name _) = "Hello, " ++ name

calculate :: [Int] -> Int
calculate xs = sum (map (*2) xs)

fibonacci :: Int -> Int
fibonacci 0 = 0
fibonacci 1 = 1
fibonacci n = fibonacci (n-1) + fibonacci (n-2)
"#,
    total_chunks: 18,
    chunk_counts: {
        Class: 2,
        Function: 12,
        Module: 3,
    }
}

test_language_extractor! {
    name: test_haskell_converter,
    language: "haskell",
    extension: "hs",
    source: r#"
data Color = Red | Green | Blue

showColor :: Color -> String
showColor Red = "red"
showColor Green = "green"
showColor Blue = "blue"

data Maybe' a = Nothing' | Just' a

instance Functor Maybe' where
    fmap _ Nothing' = Nothing'
    fmap f (Just' x) = Just' (f x)
"#,
    total_chunks: 10,
    chunk_counts: {
        Class: 2,
        Function: 7,
    }
}

test_language_extractor! {
    name: test_haskell_complex_algorithm_extraction,
    language: "haskell",
    extension: "hs",
    source: r#"
import qualified Data.Map as Map
import Data.List (groupBy, sortBy)
import Data.Function (on)

data ProcessedItem = ProcessedItem
    { itemId :: Int
    , originalValue :: Int
    , transformedValue :: Int
    , category :: String
    , timestamp :: String
    } deriving (Show, Eq)

type ItemCache = Map.Map String ProcessedItem

processComplexData :: [Int] -> Int -> [ProcessedItem]
processComplexData input threshold = processWithCache input threshold Map.empty []
  where
    processWithCache [] _ _ acc = reverse acc
    processWithCache (value:rest) thresh cache acc =
        let cacheKey = "item_" ++ show (length acc) ++ "_" ++ show value
        in case Map.lookup cacheKey cache of
            Just cachedItem -> processWithCache rest thresh cache (cachedItem : acc)
            Nothing ->
                -- Main processing algorithm - extractable middle chunk
                let processedItem = if value > thresh
                        then let transformed = value * 2
                                 cat = if transformed > thresh * 3 then "HIGH" else "MEDIUM"
                                 bonusValue = if transformed > 100 then transformed + 10 else transformed
                             in ProcessedItem (length acc) value bonusValue cat "now"
                        else if value > 0
                        then ProcessedItem (length acc) value (value + thresh) "LOW" "now"
                        else ProcessedItem (length acc) value 0 "INVALID" "now"

                    newCache = Map.insert cacheKey processedItem cache
                in if category processedItem == "INVALID"
                   then processWithCache rest thresh newCache acc
                   else processWithCache rest thresh newCache (processedItem : acc)

analyzePatterns :: [ProcessedItem] -> Map.Map String (Map.Map String Double)
analyzePatterns items =
    let categoryGroups = groupBy ((==) `on` category) $ sortBy (compare `on` category) items
    in Map.fromList $ map analyzeCategory categoryGroups
  where
    analyzeCategory group@(firstItem:_) =
        let cat = category firstItem
            values = map (fromIntegral . transformedValue) group
            count = fromIntegral $ length group
            totalItems = fromIntegral $ length items

            -- Pattern analysis logic - extractable middle chunk
            avgValue = sum values / count
            minValue = minimum values
            maxValue = maximum values
            percentage = (count / totalItems) * 100

            highValueItems = filter (> 1000) values
            highValueCount = fromIntegral $ length highValueItems

            recentItems = filter (\item -> timestamp item == "now") group
            recentCount = fromIntegral $ length recentItems

            analysis = Map.fromList
                [ ("count", count)
                , ("percentage", percentage)
                , ("avg_value", avgValue)
                , ("min_value", minValue)
                , ("max_value", maxValue)
                , ("high_value_count", highValueCount)
                , ("recent_count", recentCount)
                ]
        in (cat, analysis)
    analyzeCategory [] = ("EMPTY", Map.empty)

-- Helper functions for complex transformations
complexTransform :: ProcessedItem -> ProcessedItem
complexTransform item =
    let newValue = case category item of
            "HIGH" -> transformedValue item * 2
            "MEDIUM" -> transformedValue item + 50
            "LOW" -> transformedValue item + 10
            _ -> transformedValue item
    in item { transformedValue = newValue }

filterAndSort :: [ProcessedItem] -> String -> [ProcessedItem]
filterAndSort items targetCategory =
    let filtered = filter (\item -> category item == targetCategory) items
        sorted = sortBy (compare `on` transformedValue) filtered
    in reverse sorted  -- highest values first

batchProcess :: [[Int]] -> Int -> [[ProcessedItem]]
batchProcess batches threshold = map (\batch -> processComplexData batch threshold) batches
"#,
    total_chunks: 57,
    chunk_counts: {
        Function: 38,
        Class: 1,
    }
}
