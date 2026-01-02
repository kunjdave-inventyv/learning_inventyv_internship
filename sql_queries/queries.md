# SQL Queries

#### 1. Write a SQL query to find the name and year of the movies. Return movie title, movie release year.
```sql
SELECT mov_title, mov_year 
FROM movie;
```

| mov_title                | mov_year |
| ------------------------ | -------- |
| Vertigo                  | 1958     |
| The Innocents            | 1961     |
| Lawrence of Arabia       | 1962     |
| The Deer Hunter          | 1978     |
| Amadeus                  | 1984     |
| Blade Runner             | 1982     |
| Eyes Wide Shut           | 1999     |
| The Usual Suspects       | 1995     |
| Chinatown                | 1974     |
| Boogie Nights            | 1997     |
| Annie Hall               | 1977     |
| Princess Mononoke        | 1997     |
| The Shawshank Redemption | 1994     |
| American Beauty          | 1999     |
| Titanic                  | 1997     |
| Good Will Hunting        | 1997     |
| Deliverance              | 1972     |
| Trainspotting            | 1996     |
| The Prestige             | 2006     |
| Donnie Darko             | 2001     |
| Slumdog Millionaire      | 2008     |
| Aliens                   | 1986     |
| Beyond the Sea           | 2004     |
| Avatar                   | 2009     |
| Seven Samurai            | 1954     |
| Spirited Away            | 2001     |
| Back to the Future       | 1985     |
| Braveheart               | 1995     |

#### 2. Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.
```sql
SELECT mov_year 
FROM movie 
WHERE mov_title = 'American Beauty';
```
| mov_year |
| -------- |
| 1999     |

#### 3. Write a SQL query to find the movie that was released in 1999. Return movie title.
```sql
SELECT mov_title
FROM movie
WHERE mov_year = 1999;
```
| mov_title       |
| --------------- |
| Eyes Wide Shut  |
| American Beauty |

#### 4. Write a SQL query to find those movies, which were released before 1998. Return movie title.
```sql
SELECT mov_title
FROM movie
WHERE mov_year < 1998;
```
| mov_title                |
| ------------------------ |
| Vertigo                  |
| The Innocents            |
| Lawrence of Arabia       |
| The Deer Hunter          |
| Amadeus                  |
| Blade Runner             |
| The Usual Suspects       |
| Chinatown                |
| Boogie Nights            |
| Annie Hall               |
| Princess Mononoke        |
| The Shawshank Redemption |
| Titanic                  |
| Good Will Hunting        |
| Deliverance              |
| Trainspotting            |
| Aliens                   |
| Seven Samurai            |
| Back to the Future       |
| Braveheart               |

#### 5. Write a SQL query to find the name of all reviewers and movies together in a single list.
```sql
SELECT rev_name AS name FROM movie_reviewer WHERE rev_name IS NOT NULL
UNION
SELECT mov_title FROM movie WHERE mov_title IS NOT NULL;
```
| name                     |
| ------------------------ |
| Seven Samurai            |
| Avatar                   |
| Brandt Sponseller        |
| American Beauty          |
| Krug Stillo              |
| Deliverance              |
| Back to the Future       |
| Victor Woeltjen          |
| Wesley S. Walker         |
| Jack Malvern             |
| Princess Mononoke        |
| Braveheart               |
| Vertigo                  |
| Donnie Darko             |
| Amadeus                  |
| Scott LeBrun             |
| Blade Runner             |
| Slumdog Millionaire      |
| The Innocents            |
| Alec Shaw                |
| The Shawshank Redemption |
| Lawrence of Arabia       |
| The Usual Suspects       |
| The Deer Hunter          |
| Josh Cates               |
| Flagrant Baronessa       |
| The Prestige             |
| Spirited Away            |
| Annie Hall               |
| Hannah Steele            |
| Titanic                  |
| Righty Sock              |
| Sasha Goldshtein         |
| Neal Wruck               |
| Paul Monks               |
| Aliens                   |
| Vincent Cadena           |
| Beyond the Sea           |
| Eyes Wide Shut           |
| Chinatown                |
| Boogie Nights            |
| Richard Adams            |
| Mike Salvati             |
| Good Will Hunting        |
| Trainspotting            |
| Simon Wright             |

#### 6. Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.
```sql
SELECT DISTINCT mr.rev_name
FROM movie_reviewer mr
JOIN movie_rating mrt ON mr.rev_id = mrt.rev_id
WHERE mrt.rev_stars >= 7 ;
```
| rev_name           |
| ------------------ |
| null               |
| Hannah Steele      |
| Righty Sock        |
| Sasha Goldshtein   |
| Jack Malvern       |
| Vincent Cadena     |
| Brandt Sponseller  |
| Flagrant Baronessa |
| Krug Stillo        |
| Mike Salvati       |
| Victor Woeltjen    |
| Simon Wright       |

#### 7. Write a SQL query to find the movies without any rating. Return movie title.
```sql
SELECT m.mov_title
FROM movie m
LEFT JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars IS NULL;
```
| mov_title     |
| ------------- |
| Chinatown     |
| Trainspotting |

#### 8. Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.
```sql
SELECT mov_title 
FROM movie 
WHERE mov_id IN (905, 907, 917);
```
No rows returned

#### 9. Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.
```sql
SELECT mov_id, mov_title, mov_year
FROM movie
WHERE mov_title LIKE '%Boogie Nights%'
ORDER BY mov_year ASC;
```
| mov_id | mov_title     | mov_year |
| ------ | ------------- | -------- |
| 10     | Boogie Nights | 1997     |

#### 10. Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.
```sql
SELECT act_id
FROM actor
WHERE act_fname = 'Woody'
AND acrt_lname = 'Allen';
```
| act_id |
| ------ |
| 11     |

#### 11. Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.
```sql
SELECT *
FROM actor
WHERE act_id IN (
    SELECT act_id
    FROM movie_cast
    WHERE mov_id = (
        SELECT mov_id
        FROM movie
        WHERE mov_title = 'Annie Hall'
    )
);
```
| act_id | act_fname | acrt_lname | act_gender |
| ------ | --------- | ---------- | ---------- |
| 11     | Woody     | Allen      | M          |

#### 12. Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.
```sql
SELECT d.dir_fname, d.dir_lname
FROM director d
WHERE d.dir_id IN (
    SELECT dir_id
    FROM movie_direction
    WHERE mov_id = (
        SELECT mov_id
        FROM movie
        WHERE mov_title = 'Eyes Wide Shut'
    )
);
```
| dir_fname | dir_lname |
| --------- | --------- |
| Stanley   | Kubrick   |

#### 13. Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.
```sql
SELECT mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country
FROM movie
WHERE mov_rel_country != 'UK';
```
| mov_title     | mov_year | mov_time | mov_dt_rel | mov_rel_country |
| ------------- | -------- | -------- | ---------- | --------------- |
| The Innocents | 1961     | 100      | 1962-02-19 | SW              |
| Annie Hall    | 1977     | 93       | 1977-04-20 | USA             |
| Seven Samurai | 1954     | 207      | 1954-04-26 | JP              |

#### 14. Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.
```sql
SELECT m.mov_title, m.mov_year, m.mov_dt_rel,
       d.dir_fname, d.dir_lname,
       a.act_fname, a.acrt_lname
FROM movie m
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
JOIN movie_cast mc ON m.mov_id = mc.mov_id
JOIN actor a ON mc.act_id = a.act_id
LEFT JOIN movie_rating r ON m.mov_id = r.mov_id
WHERE r.rev_id IS NULL;
```
| mov_title                | mov_year | mov_dt_rel | dir_fname | dir_lname | act_fname | acrt_lname |
| ------------------------ | -------- | ---------- | --------- | --------- | --------- | ---------- |
| The Deer Hunter          | 1978     | 1979-03-08 | Michael   | Cimino    | Robert    | De Niro    |
| Amadeus                  | 1984     | 1985-01-07 | Milos     | Forman    | F. Murray | Abraham    |
| Eyes Wide Shut           | 1999     | null       | Stanley   | Kubrick   | Nicole    | Kidman     |
| The Shawshank Redemption | 1994     | 1995-02-17 | Frank     | Darabont  | Tim       | Robbins    |
| Deliverance              | 1972     | 1982-10-05 | John      | Boorman   | Jon       | Voight     |

#### 15. Write a SQL query to find those movies directed by the director whose first name is Woody and last name is Allen. Return movie title.
```sql
SELECT mov_title
FROM movie
WHERE mov_id IN (
    SELECT mov_id
    FROM movie_direction
    WHERE dir_id = (
        SELECT dir_id
        FROM director
        WHERE dir_fname = 'Woody'
          AND dir_lname = 'Allen'
    )
);
```
| mov_title  |
| ---------- |
| Annie Hall |

#### 16. Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.
```sql
SELECT DISTINCT mov_year
FROM movie
WHERE mov_id IN (
    SELECT mov_id
    FROM movie_rating
    WHERE rev_stars >= 3
)
ORDER BY mov_year ASC;
```
| mov_year |
| -------- |
| 1954     |
| 1958     |
| 1961     |
| 1962     |
| 1977     |
| 1982     |
| 1986     |
| 1995     |
| 1997     |
| 1999     |
| 2001     |
| 2004     |
| 2008     |
| 2009     |

#### 17. Write a SQL query to search for movies that do not have any ratings. Return movie title.
```sql
SELECT mov_title
FROM movie
WHERE mov_id NOT IN (
    SELECT mov_id 
    FROM movie_rating
);
```
| mov_title                |
| ------------------------ |
| The Deer Hunter          |
| Amadeus                  |
| Eyes Wide Shut           |
| The Shawshank Redemption |
| Deliverance              |
| The Prestige             |
| Spirited Away            |
| Back to the Future       |
| Braveheart               |

#### 18. Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.
```sql
SELECT rev_name
FROM movie_reviewer
WHERE rev_id NOT IN (
    SELECT rev_id 
    FROM movie_rating
);
```
| rev_name         |
| ---------------- |
| Alec Shaw        |
| Wesley S. Walker |

#### 19. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.
```sql
SELECT r.rev_name, m.mov_title, rt.rev_stars
FROM movie_reviewer r
JOIN movie_rating rt ON r.rev_id = rt.rev_id
JOIN movie m ON rt.mov_id = m.mov_id
ORDER BY r.rev_name, m.mov_title, rt.rev_stars;
```
| rev_name           | mov_title           | rev_stars |
| ------------------ | ------------------- | --------- |
| Brandt Sponseller  | Aliens              | 8.4       |
| Flagrant Baronessa | Lawrence of Arabia  | 8.3       |
| Hannah Steele      | Donnie Darko        | 8.1       |
| Jack Malvern       | The Innocents       | 7.9       |
| Josh Cates         | Good Will Hunting   | 4.0       |
| Krug Stillo        | Seven Samurai       | 7.7       |
| Mike Salvati       | Annie Hall          | 8.1       |
| Neal Wruck         | Chinatown           | null      |
| Paul Monks         | Boogie Nights       | 3.0       |
| Richard Adams      | Beyond the Sea      | 6.7       |
| Righty Sock        | Titanic             | 7.7       |
| Righty Sock        | Vertigo             | 8.4       |
| Sasha Goldshtein   | American Beauty     | 7.0       |
| Scott LeBrun       | Trainspotting       | null      |
| Simon Wright       | The Usual Suspects  | 8.6       |
| Victor Woeltjen    | Avatar              | 7.3       |
| Vincent Cadena     | Slumdog Millionaire | 8.0       |
| null               | Blade Runner        | 8.2       |
| null               | Princess Mononoke   | 8.4       |

#### 20. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer's name, movie title. Return reviewer's name, movie title.
```sql
SELECT r.rev_name, m.mov_title
FROM movie_reviewer r
JOIN movie_rating rt ON r.rev_id = rt.rev_id
JOIN movie m ON rt.mov_id = m.mov_id
GROUP BY r.rev_name, m.mov_title;
```
| rev_name           | mov_title           |
| ------------------ | ------------------- |
| Neal Wruck         | Chinatown           |
| Richard Adams      | Beyond the Sea      |
| null               | Princess Mononoke   |
| null               | Blade Runner        |
| Vincent Cadena     | Slumdog Millionaire |
| Righty Sock        | Titanic             |
| Sasha Goldshtein   | American Beauty     |
| Brandt Sponseller  | Aliens              |
| Krug Stillo        | Seven Samurai       |
| Righty Sock        | Vertigo             |
| Hannah Steele      | Donnie Darko        |
| Josh Cates         | Good Will Hunting   |
| Mike Salvati       | Annie Hall          |
| Simon Wright       | The Usual Suspects  |
| Flagrant Baronessa | Lawrence of Arabia  |
| Jack Malvern       | The Innocents       |
| Paul Monks         | Boogie Nights       |
| Victor Woeltjen    | Avatar              |
| Scott LeBrun       | Trainspotting       |

#### 21. Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.
```sql
SELECT m.mov_title, MAX(rt.rev_stars) AS max_stars
FROM movie m
JOIN movie_rating rt ON rt.mov_id = m.mov_id
GROUP BY m.mov_title
ORDER BY m.mov_title ASC;
```
| mov_title           | max  |
| ------------------- | ---- |
| Aliens              | 8.4  |
| American Beauty     | 7.0  |
| Annie Hall          | 8.1  |
| Avatar              | 7.3  |
| Beyond the Sea      | 6.7  |
| Blade Runner        | 8.2  |
| Boogie Nights       | 3.0  |
| Chinatown           | null |
| Donnie Darko        | 8.1  |
| Good Will Hunting   | 4.0  |
| Lawrence of Arabia  | 8.3  |
| Princess Mononoke   | 8.4  |
| Seven Samurai       | 7.7  |
| Slumdog Millionaire | 8.0  |
| The Innocents       | 7.9  |
| The Usual Suspects  | 8.6  |
| Titanic             | 7.7  |
| Trainspotting       | null |
| Vertigo             | 8.4  |

#### 22. Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.
```sql
SELECT rv.rev_name 
FROM movie_reviewer rv 
WHERE rv.rev_id IN ( 
    SELECT rt.rev_id 
    FROM movie_rating rt 
    WHERE rt.mov_id = (
        SELECT m.mov_id 
        FROM movie m 
        WHERE m.mov_title = 'American Beauty'
    )
);
```
| rev_name         |
| ---------------- |
| Sasha Goldshtein |

#### 23. Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.
```sql
SELECT m.mov_title
FROM movie m
WHERE m.mov_id NOT IN (
    SELECT r.mov_id
    FROM movie_rating r
    JOIN movie_reviewer mr ON r.rev_id = mr.rev_id
    WHERE mr.rev_name != 'Paul Monks'
);
```
| mov_title                |
| ------------------------ |
| The Deer Hunter          |
| Amadeus                  |
| Blade Runner             |
| Eyes Wide Shut           |
| Boogie Nights            |
| Princess Mononoke        |
| The Shawshank Redemption |
| Deliverance              |
| The Prestige             |
| Spirited Away            |
| Back to the Future       |
| Braveheart               |

#### 24. Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.
```sql
SELECT r.rev_name,
       m.mov_title,
       mr.rev_stars
FROM movie_rating mr
JOIN movie m ON mr.mov_id = m.mov_id
JOIN movie_reviewer r ON mr.rev_id = r.rev_id
WHERE mr.rev_stars = (
    SELECT MIN(rev_stars)
    FROM movie_rating
    WHERE rev_stars IS NOT NULL
);
```
| rev_name   | mov_title     | rev_stars |
| ---------- | ------------- | --------- |
| Paul Monks | Boogie Nights | 3.0       |

#### 25. Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.
```sql
SELECT m.mov_title 
FROM movie m 
WHERE m.mov_id IN (
    SELECT md.mov_id 
    FROM movie_direction md 
    WHERE md.dir_id IN (
        SELECT d.dir_id 
        FROM director d 
        WHERE d.dir_fname = 'James' 
          AND d.dir_lname = 'Cameron'
    )
);
```
| mov_title |
| --------- |
| Aliens    |
| Titanic   |

#### 26. Write a query in SQL to find the movies in which one or more actors appeared in more than one film.
```sql
SELECT DISTINCT m.mov_title
FROM movie m
JOIN movie_cast mc ON m.mov_id = mc.mov_id
WHERE mc.act_id IN (
    SELECT act_id
    FROM movie_cast
    GROUP BY act_id
    HAVING COUNT(DISTINCT mov_id) > 1
);
```
| mov_title       |
| --------------- |
| American Beauty |
| Beyond the Sea  |

#### 27. Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.
```sql
SELECT mr.rev_name
FROM movie_reviewer mr
JOIN movie_rating rt ON rt.rev_id = mr.rev_id
WHERE rt.rev_stars IS NULL;
```
| rev_name     |
| ------------ |
| Neal Wruck   |
| Scott LeBrun |

#### 28. Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.
```sql
SELECT a.act_fname, a.acrt_lname, mc.role
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
WHERE m.mov_title = 'Annie Hall';
```
| act_fname | acrt_lname | role        |
| --------- | ---------- | ----------- |
| Woody     | Allen      | Alvy Singer |

#### 29. Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title
FROM director d
JOIN movie_direction md ON d.dir_id = md.dir_id
JOIN movie m ON md.mov_id = m.mov_id
WHERE m.mov_title = 'Eyes Wide Shut';
```
| dir_fname | dir_lname | mov_title      |
| --------- | --------- | -------------- |
| Stanley   | Kubrick   | Eyes Wide Shut |

#### 30. Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title
FROM director d
JOIN movie_direction md ON d.dir_id = md.dir_id
JOIN movie m ON md.mov_id = m.mov_id
JOIN movie_cast mc ON m.mov_id = mc.mov_id
WHERE mc.role = 'Sean Maguire';
```
| dir_fname | dir_lname | mov_title         |
| --------- | --------- | ----------------- |
| Gus       | Van Sant  | Good Will Hunting |

#### 31. Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.
```sql
SELECT a.act_fname, a.acrt_lname, m.mov_title, m.mov_year
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
WHERE a.act_id NOT IN (
    SELECT DISTINCT mc2.act_id
    FROM movie_cast mc2
    JOIN movie m2 ON mc2.mov_id = m2.mov_id
    WHERE m2.mov_year BETWEEN 1990 AND 2000
);
```
| act_fname | acrt_lname | mov_title           | mov_year |
| --------- | ---------- | ------------------- | -------- |
| James     | Stewart    | Vertigo             | 1958     |
| Deborah   | Kerr       | The Innocents       | 1961     |
| Peter     | OToole     | Lawrence of Arabia  | 1962     |
| Robert    | De Niro    | The Deer Hunter     | 1978     |
| F. Murray | Abraham    | Amadeus             | 1984     |
| Harrison  | Ford       | Blade Runner        | 1982     |
| Woody     | Allen      | Annie Hall          | 1977     |
| Jon       | Voight     | Deliverance         | 1972     |
| Maggie    | Gyllenhaal | Donnie Darko        | 2001     |
| Dev       | Patel      | Slumdog Millionaire | 2008     |
| Sigourney | Weaver     | Aliens              | 1986     |
| Jack      | Nicholson  | Chinatown           | 1974     |
| Christian | Bale       | Chinatown           | 1974     |

#### 32. Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.
```sql
SELECT d.dir_fname, d.dir_lname, g.gen_title, COUNT(DISTINCT m.mov_id) AS number_of_genre_movies
FROM director d
JOIN movie_direction md ON d.dir_id = md.dir_id
JOIN movie m ON md.mov_id = m.mov_id
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
GROUP BY d.dir_fname, d.dir_lname, g.gen_title;
```
| dir_fname | dir_lname | gen_title | number_of_genre_movies |
| --------- | --------- | --------- | ---------------------- |
| Alfred    | Hitchcock | Mystery   | 1                      |
| Bryan     | Singer    | Crime     | 1                      |
| Danny     | Boyle     | Drama     | 2                      |
| David     | Lean      | Adventure | 1                      |
| Frank     | Darabont  | Crime     | 1                      |
| Hayao     | Miyazaki  | Animation | 1                      |
| Jack      | Clayton   | Horror    | 1                      |
| James     | Cameron   | Action    | 1                      |
| John      | Boorman   | Adventure | 1                      |
| Kevin     | Spacey    | Music     | 1                      |
| Michael   | Cimino    | War       | 1                      |
| Ridley    | Scott     | Thriller  | 1                      |
| Sam       | Mendes    | Romance   | 1                      |
| Stanley   | Kubrick   | Mystery   | 1                      |
| Woody     | Allen     | Comedy    | 1                      |

#### 33. Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.
```sql
SELECT m.mov_title, m.mov_year, g.gen_title
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id;
```
| mov_title                | mov_year | gen_title |
| ------------------------ | -------- | --------- |
| Aliens                   | 1986     | Action    |
| Deliverance              | 1972     | Adventure |
| Lawrence of Arabia       | 1962     | Adventure |
| Princess Mononoke        | 1997     | Animation |
| Annie Hall               | 1977     | Comedy    |
| The Usual Suspects       | 1995     | Crime     |
| The Shawshank Redemption | 1994     | Crime     |
| Spirited Away            | 2001     | Drama     |
| Braveheart               | 1995     | Drama     |
| Trainspotting            | 1996     | Drama     |
| Slumdog Millionaire      | 2008     | Drama     |
| The Innocents            | 1961     | Horror    |
| Beyond the Sea           | 2004     | Music     |
| Eyes Wide Shut           | 1999     | Mystery   |
| Back to the Future       | 1985     | Mystery   |
| Vertigo                  | 1958     | Mystery   |
| American Beauty          | 1999     | Romance   |
| Blade Runner             | 1982     | Thriller  |
| The Deer Hunter          | 1978     | War       |

#### 34. Write a SQL query to find all the movies with year, genres, and name of the director.
```sql
SELECT m.mov_title, m.mov_year, g.gen_title, d.dir_fname, d.dir_lname
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id;
```
| mov_title                | mov_year | gen_title | dir_fname | dir_lname |
| ------------------------ | -------- | --------- | --------- | --------- |
| Aliens                   | 1986     | Action    | James     | Cameron   |
| Deliverance              | 1972     | Adventure | John      | Boorman   |
| Lawrence of Arabia       | 1962     | Adventure | David     | Lean      |
| Princess Mononoke        | 1997     | Animation | Hayao     | Miyazaki  |
| Annie Hall               | 1977     | Comedy    | Woody     | Allen     |
| The Usual Suspects       | 1995     | Crime     | Bryan     | Singer    |
| The Shawshank Redemption | 1994     | Crime     | Frank     | Darabont  |
| Trainspotting            | 1996     | Drama     | Danny     | Boyle     |
| Slumdog Millionaire      | 2008     | Drama     | Danny     | Boyle     |
| The Innocents            | 1961     | Horror    | Jack      | Clayton   |
| Beyond the Sea           | 2004     | Music     | Kevin     | Spacey    |
| Eyes Wide Shut           | 1999     | Mystery   | Stanley   | Kubrick   |
| Vertigo                  | 1958     | Mystery   | Alfred    | Hitchcock |
| American Beauty          | 1999     | Romance   | Sam       | Mendes    |
| Blade Runner             | 1982     | Thriller  | Ridley    | Scott     |
| The Deer Hunter          | 1978     | War       | Michael   | Cimino    |

#### 35. Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.
```sql
SELECT m.mov_title, m.mov_year, m.mov_dt_rel, m.mov_time, d.dir_fname, d.dir_lname
FROM movie m
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
WHERE m.mov_dt_rel < '1989-01-01'
ORDER BY m.mov_dt_rel DESC;
```
| mov_title          | mov_year | mov_dt_rel | mov_time | dir_fname | dir_lname |
| ------------------ | -------- | ---------- | -------- | --------- | --------- |
| Aliens             | 1986     | 1986-08-29 | 137      | James     | Cameron   |
| Amadeus            | 1984     | 1985-01-07 | 160      | Milos     | Forman    |
| Deliverance        | 1972     | 1982-10-05 | 109      | John      | Boorman   |
| Blade Runner       | 1982     | 1982-09-09 | 117      | Ridley    | Scott     |
| The Deer Hunter    | 1978     | 1979-03-08 | 183      | Michael   | Cimino    |
| Annie Hall         | 1977     | 1977-04-20 | 93       | Woody     | Allen     |
| Chinatown          | 1974     | 1974-08-09 | 130      | Roman     | Polanski  |
| Lawrence of Arabia | 1962     | 1962-12-11 | 216      | David     | Lean      |
| The Innocents      | 1961     | 1962-02-19 | 100      | Jack      | Clayton   |
| Vertigo            | 1958     | 1958-08-24 | 128      | Alfred    | Hitchcock |

#### 36. Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.
```sql
SELECT g.gen_title, AVG(m.mov_time) AS average_time, COUNT(m.mov_id) AS number_of_movies
FROM genres g
INNER JOIN movie_genres mg ON g.gen_id = mg.gen_id
INNER JOIN movie m ON mg.mov_id = m.mov_id
GROUP BY g.gen_title
ORDER BY g.gen_title;
```
| gen_title | average_time         | number_of_movies |
| --------- | -------------------- | ---------------- |
| Action    | 137.0000000000000000 | 1                |
| Adventure | 162.5000000000000000 | 2                |
| Animation | 134.0000000000000000 | 1                |
| Comedy    | 93.0000000000000000  | 1                |
| Crime     | 124.0000000000000000 | 2                |
| Drama     | 129.2500000000000000 | 4                |
| Horror    | 100.0000000000000000 | 1                |
| Music     | 118.0000000000000000 | 1                |
| Mystery   | 134.3333333333333333 | 3                |
| Romance   | 122.0000000000000000 | 1                |
| Thriller  | 117.0000000000000000 | 1                |
| War       | 183.0000000000000000 | 1                |

#### 37. Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.
```sql
SELECT m.mov_title, m.mov_year, d.dir_fname, d.dir_lname, a.act_fname, a.acrt_lname, mc.role
FROM movie m
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE m.mov_time = (SELECT MIN(mov_time) FROM movie);
```
| mov_title  | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role        |
| ---------- | -------- | --------- | --------- | --------- | ---------- | ----------- |
| Annie Hall | 1977     | Woody     | Allen     | Woody     | Allen      | Alvy Singer |

#### 38. Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.
```sql
SELECT DISTINCT m.mov_year
FROM movie m
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars IN (3, 4)
ORDER BY m.mov_year ASC;
```
| mov_year |
| -------- |
| 1997     |

#### 39. Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.
```sql
SELECT mr_rev.rev_name, m.mov_title, mr.rev_stars
FROM movie_rating mr
INNER JOIN movie m ON mr.mov_id = m.mov_id
INNER JOIN movie_reviewer mr_rev ON mr.rev_id = mr_rev.rev_id
ORDER BY mr_rev.rev_name ASC, m.mov_title ASC, mr.rev_stars ASC;
```
| rev_name           | mov_title           | rev_stars |
| ------------------ | ------------------- | --------- |
| Brandt Sponseller  | Aliens              | 8.4       |
| Flagrant Baronessa | Lawrence of Arabia  | 8.3       |
| Hannah Steele      | Donnie Darko        | 8.1       |
| Jack Malvern       | The Innocents       | 7.9       |
| Josh Cates         | Good Will Hunting   | 4.0       |
| Krug Stillo        | Seven Samurai       | 7.7       |
| Mike Salvati       | Annie Hall          | 8.1       |
| Neal Wruck         | Chinatown           | null      |
| Paul Monks         | Boogie Nights       | 3.0       |
| Richard Adams      | Beyond the Sea      | 6.7       |
| Righty Sock        | Titanic             | 7.7       |
| Righty Sock        | Vertigo             | 8.4       |
| Sasha Goldshtein   | American Beauty     | 7.0       |
| Scott LeBrun       | Trainspotting       | null      |
| Simon Wright       | The Usual Suspects  | 8.6       |
| Victor Woeltjen    | Avatar              | 7.3       |
| Vincent Cadena     | Slumdog Millionaire | 8.0       |
| null               | Blade Runner        | 8.2       |
| null               | Princess Mononoke   | 8.4       |

#### 40. Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.
```sql
SELECT m.mov_title, mr.rev_stars AS max_stars
FROM movie m
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars = (SELECT MAX(rev_stars) FROM movie_rating)
ORDER BY m.mov_title ASC;
```
| mov_title          | max_stars |
| ------------------ | --------- |
| The Usual Suspects | 8.6       |

#### 41. Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.
```sql
SELECT m.mov_title, d.dir_fname, d.dir_lname, mr.rev_stars
FROM movie m
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
ORDER BY m.mov_title, mr.rev_stars;
```
| mov_title           | dir_fname | dir_lname       | rev_stars |
| ------------------- | --------- | --------------- | --------- |
| Aliens              | James     | Cameron         | 8.4       |
| American Beauty     | Sam       | Mendes          | 7.0       |
| Annie Hall          | Woody     | Allen           | 8.1       |
| Beyond the Sea      | Kevin     | Spacey          | 6.7       |
| Blade Runner        | Ridley    | Scott           | 8.2       |
| Boogie Nights       | Paul      | Thomas Anderson | 3.0       |
| Chinatown           | Roman     | Polanski        | null      |
| Donnie Darko        | Richard   | Kelly           | 8.1       |
| Good Will Hunting   | Gus       | Van Sant        | 4.0       |
| Lawrence of Arabia  | David     | Lean            | 8.3       |
| Princess Mononoke   | Hayao     | Miyazaki        | 8.4       |
| Slumdog Millionaire | Danny     | Boyle           | 8.0       |
| The Innocents       | Jack      | Clayton         | 7.9       |
| The Usual Suspects  | Bryan     | Singer          | 8.6       |
| Titanic             | James     | Cameron         | 7.7       |
| Trainspotting       | Danny     | Boyle           | null      |
| Vertigo             | Alfred    | Hitchcock       | 8.4       |

#### 42. Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.
```sql
SELECT m.mov_title, a.act_fname, a.acrt_lname, mc.role
FROM movie m
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE mc.act_id IN (
    SELECT act_id
    FROM movie_cast
    GROUP BY act_id
    HAVING COUNT(DISTINCT mov_id) > 1
);
```
| mov_title       | act_fname | acrt_lname | role           |
| --------------- | --------- | ---------- | -------------- |
| American Beauty | Kevin     | Spacey     | Lester Burnham |
| Beyond the Sea  | Kevin     | Spacey     | Bobby Darin    |

#### 43. Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title, a.act_fname, a.acrt_lname, mc.role
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
WHERE a.act_fname = 'Claire' AND a.acrt_lname = 'Danes';
```
| dir_fname | dir_lname | mov_title         | act_fname | acrt_lname | role |
| --------- | --------- | ----------------- | --------- | ---------- | ---- |
| Hayao     | Miyazaki  | Princess Mononoke | Claire    | Danes      | San  |

#### 44. Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.
```sql
SELECT a.act_fname, a.acrt_lname, m.mov_title, mc.role
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
WHERE a.act_fname = d.dir_fname AND a.acrt_lname = d.dir_lname;
```
| act_fname | acrt_lname | mov_title      | role        |
| --------- | ---------- | -------------- | ----------- |
| Woody     | Allen      | Annie Hall     | Alvy Singer |
| Kevin     | Spacey     | Beyond the Sea | Bobby Darin |

#### 45. Write a SQL query to find the cast list of the movie 'Chinatown'. Return first name, last name.
```sql
SELECT a.act_fname, a.acrt_lname
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
WHERE m.mov_title = 'Chinatown';
```
| act_fname | acrt_lname |
| --------- | ---------- |
| Jack      | Nicholson  |
| Christian | Bale       |

#### 46. Write a SQL query to find those movies where actor's first name is 'Harrison' and last name is 'Ford'. Return movie title.
```sql
SELECT m.mov_title
FROM movie m
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE a.act_fname = 'Harrison' AND a.acrt_lname = 'Ford';
```
| mov_title    |
| ------------ |
| Blade Runner |

#### 47. Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.
```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars, m.mov_rel_country
FROM movie m
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars = (SELECT MAX(rev_stars) FROM movie_rating);
```
| mov_title          | mov_year | rev_stars | mov_rel_country |
| ------------------ | -------- | --------- | --------------- |
| The Usual Suspects | 1995     | 8.6       | UK              |

#### 48. Write a SQL query to find the highest-rated 'Mystery Movies'. Return the title, year, and rating.
```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars
FROM movie m
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
WHERE g.gen_title = 'Mystery' AND mr.rev_stars = (
    SELECT MAX(mr2.rev_stars)
    FROM movie_rating mr2
    INNER JOIN movie_genres mg2 ON mr2.mov_id = mg2.mov_id
    INNER JOIN genres g2 ON mg2.gen_id = g2.gen_id
    WHERE g2.gen_title = 'Mystery'
);
```
| mov_title | mov_year | rev_stars |
| --------- | -------- | --------- |
| Vertigo   | 1958     | 8.4       |

#### 49. Write a SQL query to find the years when most of the 'Mystery Movies' produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.
```sql
SELECT m.mov_year, g.gen_title, COUNT(g.gen_title) AS number_of_genre_title, AVG(mr.rev_stars) AS average_rating
FROM movie m
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE g.gen_title = 'Mystery'
GROUP BY m.mov_year, g.gen_title;
```
| mov_year | gen_title | number_of_genre_title | average_rating     |
| -------- | --------- | --------------------- | ------------------ |
| 1958     | Mystery   | 1                     | 8.4000000000000000 |

#### 50. Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.
```sql
SELECT m.mov_title, a.act_fname, a.acrt_lname, m.mov_year, mc.role, g.gen_title, d.dir_fname, d.dir_lname, m.mov_dt_rel, mr.rev_stars
FROM movie m
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
INNER JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE a.act_gender = 'F';
```
| mov_title         | act_fname | acrt_lname | mov_year | role         | gen_title | dir_fname | dir_lname | mov_dt_rel | rev_stars |
| ----------------- | --------- | ---------- | -------- | ------------ | --------- | --------- | --------- | ---------- | --------- |
| The Innocents     | Deborah   | Kerr       | 1961     | Miss Giddens | Horror    | Jack      | Clayton   | 1962-02-19 | 7.9       |
| Princess Mononoke | Claire    | Danes      | 1997     | San          | Animation | Hayao     | Miyazaki  | 2001-10-19 | 8.4       |
| Aliens            | Sigourney | Weaver     | 1986     | Ripley       | Action    | James     | Cameron   | 1986-08-29 | 8.4       |