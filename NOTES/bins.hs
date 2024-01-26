-- Code for bins agglomerating multiple points.

data Bin = Bin { binCount :: Int, binSum :: Double, binSumSquare :: Double, binMin :: Double, binMax :: Double }
         deriving (Eq, Show)

binSingle :: Double -> Bin
binSingle x = Bin 1 x (x^2) x x

binMerge :: Bin -> Bin -> Bin
binMerge (Bin n1 s1 ss1 min1 max1)
         (Bin n2 s2 ss2 min2 max2)
  = Bin (n1+n2) (s1+s2) (ss1+ss2) (min min1 min2) (max max1 max2)

binMean :: Bin -> Double
binMean b = binSum b / fromIntegral (binCount b)

binVar :: Bin -> Double
binVar b = binSumSquare b / fromIntegral (binCount b) - binMean b^2

binStdev :: Bin -> Double
binStdev b = sqrt (binVar b)

binRange :: Bin -> (Double, Double)
binRange b = (binMin b, binMax b)

-- Test
-- ghci> :l bins.hs
-- ghci> b = foldl1 binMerge (map binSingle [0..9])
-- ghci> binMean b
-- 4.5 :: Double
-- ghci> binStdev b
-- 2.87228
