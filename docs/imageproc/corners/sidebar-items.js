initSidebarItems({"enum":[["Fast","Variants of the FAST corner detector. These classify a point based on its intensity relative to the 16 pixels in the Bresenham circle of radius 3 around it. A point P with intensity I is detected as a corner if all pixels in a sufficiently long contiguous section of this circle either all have intensity greater than I + t or all have intensity less than I - t, for some user-provided threshold t. The score of a corner is the greatest threshold for which the given pixel still qualifies as a corner."]],"fn":[["corners_fast12","Finds corners using FAST-12 features. See comment on `Fast`."],["corners_fast9","Finds corners using FAST-9 features. See comment on Fast enum."],["fast_corner_score","The score of a corner detected using the FAST detector is the largest threshold for which this pixel is still a corner. We input the threshold at which the corner was detected as a lower bound on the search. Note that the corner check uses a strict inequality, so if the smallest intensity difference between the center pixel and a corner pixel is n then the corner will have a score of n - 1."]],"struct":[["Corner","A location and score for a detected corner. The scores need not be comparable between different corner detectors."]]});